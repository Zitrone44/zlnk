use dotenv::dotenv;
use regex::Regex;
use std::env;
use std::str::FromStr;

pub struct Env {
    pub redis_url: String,
    pub url_regex: Regex,
    pub short_length: usize,
    pub short_alphabet: String,
    pub bad_request_message: String,
    pub mmdb_path: String,
    pub trust_proxy: bool,
    pub disable_stats: bool
}

const URL_REGEX: &'static str = r"^(https?://)?([\da-z\.-]+)\.([a-z\.]{2,6})([/\w \.-]*)*/?$";

pub fn init() -> Env {
    dotenv().ok();
    Env {
        redis_url: env::var("REDIS_URL").unwrap_or("redis://localhost".to_string()),
        url_regex: Regex::new(&env::var("URL_REGEX").unwrap_or(URL_REGEX.to_string())).unwrap(),
        short_length: usize::from_str(&env::var("SHORT_LENGTH").unwrap_or("5".to_string())).unwrap(),
        short_alphabet: env::var("SHORT_ALPHABET").unwrap_or("hex".to_string()),
        bad_request_message: env::var("BAD_REQUEST_MESSAGE").unwrap_or("Ungültige URL".to_string()),
        mmdb_path: env::var("MMDB_PATH").unwrap_or("./GeoLite2-Country.mmdb".to_string()),
        trust_proxy: env::var("TRUST_PROXY").is_ok(),
        disable_stats: env::var("DISABLE_STATS").is_ok()
    }
}
