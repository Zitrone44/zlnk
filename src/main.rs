#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate dotenv;
extern crate regex;
extern crate rand;
extern crate rocksdb;

mod env_loader;
mod shortener;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::Request;
use rocket::response::Content;
use rocket::response::Redirect;
use rocket::response::Failure;
use rocket::State;
use rocksdb::DB;
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
static_file!("/assets/link-intact.svg", "../assets/link-intact.svg", ContentType::SVG, link_icon);

#[post("/shorten", data="<long_url>")]
fn shorten(long_url: String, env: State<Env>, db: State<DB>) -> Result<String, Failure> {
    let short_url = short(long_url, env.inner(), db.inner());
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
fn longen(short: String, db: State<DB>) -> Option<Redirect> {
    let long = long(short, db.inner());
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
    let db = DB::open_default(&env.database_path).unwrap();
    rocket::ignite()
        .mount("/", routes![index, js, jquery, bootstrap_css, bootstrap_js, link_icon, shorten, longen])
        .catch(errors![not_found, bad_request]).manage(env).manage(db)
        .launch();
}
