use maxminddb::{geoip2, Reader};
use std::net::IpAddr;
use std::str::FromStr;

pub struct GeoLocateIP {
    reader: Reader
}

impl GeoLocateIP {
    pub fn new(path: String) -> Self {
        let reader = Reader::open(&path).unwrap();
        GeoLocateIP {reader: reader}
    }
    pub fn locate(&self, ip_string: String) -> Option<String> {
        let ip: IpAddr = FromStr::from_str(&ip_string).unwrap();
        let country = self.reader.lookup(ip);
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