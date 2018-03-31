#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate dotenv;
extern crate regex;
extern crate rand;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

mod env_loader;
mod shortener;
mod test;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::Request;
use rocket::response::Content;
use rocket::response::Redirect;
use rocket::response::Failure;
use rocket::State;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use env_loader::Env;
use shortener::{short, long};

macro_rules! static_file {
    ($route:expr, $path:expr, $type:expr, $func:ident) => (
        #[get($route)]
        fn $func() -> Content<Vec<u8>> {
            Content($type, include_bytes!($path).to_vec())
        }
    )
}
static_file!("/", "../static/index.html", ContentType::HTML, index);
static_file!("/zlnk.js", "../static/zlnk.js", ContentType::JavaScript, js);
static_file!("/assets/jquery.min.js", "../assets/jquery.min.js", ContentType::JavaScript, jquery);
static_file!("/assets/bootstrap.min.css", "../assets/bootstrap.min.css", ContentType::CSS, bootstrap_css);
static_file!("/assets/bootstrap.min.js", "../assets/bootstrap.min.js", ContentType::JavaScript, bootstrap_js);

#[post("/shorten", data="<long_url>")]
fn shorten(long_url: String, env: State<Env>, pool: State<Pool<RedisConnectionManager>>) -> Result<String, Failure> {
    let connection = &pool.get().unwrap();
    let short_url = short(long_url, env.inner(), connection);
    match short_url {
        Some(short_url) => {
            Ok(short_url)
        }
        None => {
            Err(Failure(Status::BadRequest))
        }
    }
}

#[get("/<short>")]
fn longen(short: String, pool: State<Pool<RedisConnectionManager>>) -> Option<Redirect> {
    let connection = &pool.get().unwrap();
    let long = long(short, connection);
    match long {
        Some(long) => {
            Some(Redirect::to(&long))
        }
        None => {
            None
        }
    }
}

#[error(404)]
fn not_found() -> Content<Vec<u8>> {
    Content(ContentType::HTML, include_bytes!("../static/404.html").to_vec())
}

#[error(400)]
fn bad_request(request: &Request) -> String {
    let env = request.guard::<State<Env>>().unwrap();
    env.bad_request_message.to_owned()
}

fn main() {
    let env = env_loader::init();
    let manager = RedisConnectionManager::new(env.redis_url.as_str()).unwrap();
    let pool = r2d2::Pool::builder()
        .build(manager)
        .unwrap();
    rocket::ignite()
        .mount("/", routes![index, js, jquery, bootstrap_css, bootstrap_js, shorten, longen])
        .catch(errors![not_found, bad_request]).manage(env).manage(pool)
        .launch();
}
