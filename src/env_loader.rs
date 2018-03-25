use dotenv::dotenv;
use regex::Regex;
use std::env;
use std::str::FromStr;

pub struct Env {
    pub database_path: String,
    pub url_regex: Regex,
    pub short_length: usize,
    pub bad_request_message: String
}

const URL_REGEX: &'static str = "^(([^:/?#]+):)?(//([^/?#]*))?([^?#]*)(\\?([^#]*))?(#(.*))?";

pub fn init() -> Env {
    dotenv().ok();
    Env {
        database_path: env::var("DATABASE_PATH").unwrap_or("./db".to_string()),
        url_regex: Regex::new(&env::var("URL_REGEX").unwrap_or(URL_REGEX.to_string())).unwrap(),
        short_length: usize::from_str(&env::var("SHORT_LENGTH").unwrap_or("5".to_string())).unwrap(),
        bad_request_message: env::var("BAD_REQUEST_MESSAGE").unwrap_or("Ungültige URL".to_string())
    }
}