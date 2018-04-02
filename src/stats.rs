use rocket::{request, Request, Outcome, State};
use rocket::request::FromRequest;
use r2d2;
use r2d2_redis;
use redis::{Commands, RedisError};
use url::Url;
use woothee::parser::Parser;
use env_loader::Env;
use geo_locate_ip::GeoLocateIP;
use serde_json::Value;
use std::collections::HashMap;

pub struct Stats {
    pub refer: String,
    pub browser: String,
    pub os: String,
    pub country: String
}

struct IP {
    ip: String
}

impl<'a, 'r> FromRequest<'a, 'r>  for IP {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<IP, ()> {
        let env = request.guard::<State<Env>>().unwrap();
        let trust_proxy = env.trust_proxy;
        let forwarded_for_vec: Vec<_> = request.headers().get("X-Forwarded-For").collect();
        if trust_proxy {
            let last = forwarded_for_vec.len() - 1;
            let forwarded_for = forwarded_for_vec[last];
            return Outcome::Success(IP {ip: forwarded_for.to_string()});
        } else {
            let ip = request.remote().unwrap().ip();
            return Outcome::Success(IP {ip: ip.to_string()});
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Stats {    
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Stats, ()> {
        let env = request.guard::<State<Env>>().unwrap();
        if env.disable_stats {
            return Outcome::Success(Stats {
                refer: "Unknown".to_string(),
                browser: "Unknown".to_string(),
                os: "Unknown".to_string(),
                country: "Unknown".to_string()
            })
        }
        let ip = request.guard::<IP>().unwrap();
        let geo_locate_ip = request.guard::<State<GeoLocateIP>>().unwrap();
        let refers: Vec<_> = request.headers().get("Referer").collect();
        let refer;
        if refers.len() == 1 {
            let refer_domain = Url::parse(refers[0]).unwrap().host_str().unwrap().to_owned();
            refer = Some(refer_domain.to_string());
        } else {
            refer = None;
        }
        let user_agents: Vec<_> = request.headers().get("User-Agent").collect();
        let browser;
        let os;
        if user_agents.len() == 1 {
            let parser = Parser::new();
            let result = parser.parse(user_agents[0]).unwrap();
            browser = Some(result.name);
            os = Some(result.os);
        } else {
            browser = None;
            os = None;
        }
        let country = geo_locate_ip.locate(ip.ip);

        return Outcome::Success(Stats {
            refer: refer.unwrap_or("Unknown".to_string()),
            browser: browser.unwrap_or("Unknown".to_string()),
            os: os.unwrap_or("Unknown".to_string()),
            country: country.unwrap_or("Unknown".to_string())
        });
    }
}

impl Stats {
    pub fn save(&self, short: String, connection: &r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>) -> Result<(), RedisError> {
        let stats_key = &format!("stats_{}", short);
        let refer_key = &format!("refer_{}", self.refer);
        let browser_key = &format!("browser_{}", self.browser);
        let os_key = &format!("os_{}", self.os);
        let country_key = &format!("country_{}", self.country);
        let clicks_exists: bool = connection.hexists(stats_key, "clicks")?;
        let refer_exists: bool = connection.hexists(stats_key, refer_key)?;
        let browser_exists: bool = connection.hexists(stats_key, browser_key)?;
        let os_exists: bool = connection.hexists(stats_key, os_key)?;
        let country_exists: bool = connection.hexists(stats_key, country_key)?;
        if clicks_exists {
            let _: () = connection.hincr(stats_key, "clicks", 1)?;
        } else {
            let _: () = connection.hset(stats_key, "clicks", 1)?;
        }
        if refer_exists {
            let _: () = connection.hincr(stats_key, refer_key, 1)?;
        } else {
            let _: () = connection.hset(stats_key, refer_key, 1)?;
        }
        if browser_exists {
            let _: () = connection.hincr(stats_key, browser_key, 1)?;
        } else {
            let _: () = connection.hset(stats_key, browser_key, 1)?;
        }
        if os_exists {
            let _: () = connection.hincr(stats_key, os_key, 1)?;
        } else {
            let _: () = connection.hset(stats_key, os_key, 1)?;
        }
        if country_exists {
            let _: () = connection.hincr(stats_key, country_key, 1)?;
        } else {
            let _: () = connection.hset(stats_key, country_key, 1)?;
        }
        Ok(())
    }
    pub fn stats_as_json(short: String, connection: &r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>) -> Result<Value, RedisError> {
        let _exists: String = connection.get(format!("short_{}", &short))?;
        let stats_key = &format!("stats_{}", short);
        let mut clicks: u64 = 0;
        let mut refers: HashMap<String, u64> = HashMap::new();
        let mut browsers: HashMap<String, u64> = HashMap::new();
        let mut oss: HashMap<String, u64> = HashMap::new();
        let mut countries: HashMap<String, u64> = HashMap::new();
        let keys: Vec<String> = connection.hkeys(stats_key)?;
        for key in keys.iter() {
            let value :u64 = connection.hget(stats_key, key)?;
            if key == "clicks" {
                clicks = value;
            } else {
                let split: Vec<_> = key.split('_').collect();
                let typ = split[0];
                let name = split[1].to_string();
                if typ == "refer" {
                    refers.insert(name, value);
                } else if typ == "browser" {
                    browsers.insert(name, value);
                } else if typ == "os" {
                    oss.insert(name, value);
                } else if typ == "country" {
                    countries.insert(name, value);
                } else {
                    panic!("Invalid typ: {}!", typ);
                }
            }
        }
        Ok(json!({
            "clicks": clicks,
            "refers": refers,
            "browsers": browsers,
            "oss": oss,
            "countries": countries
        }))
    }
}