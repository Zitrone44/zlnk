use maxminddb::{geoip2, Reader};
use std::net::IpAddr;
use std::str::FromStr;

pub struct GeoLocateIP {
    reader: Option<Reader>
}

impl GeoLocateIP {
    pub fn new(path: String, enabled: bool) -> Self {
        if enabled {
            let reader = Reader::open(&path).expect(format!("Failed to Load \"{}\"!", &path).as_str());
            GeoLocateIP {reader: Some(reader)}
        } else {
            GeoLocateIP {reader: None}
        }
    }
    pub fn locate(&self, ip_string: String) -> Option<String> {
        let ip: IpAddr = FromStr::from_str(&ip_string).unwrap();
        let country;
        match self.reader {
            Some(ref reader) => {
                country = reader.lookup(ip);
            }
            None => {
                panic!("GEOReader not enabled!");
            }
        }
        match country {
            Ok(result) => {
                let country: geoip2::Country = result;
                Some(country.country.unwrap().iso_code.unwrap())
            }
            Err(_err) => {
                None
            }
        }
    }
}