#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
use rocket::figment::{providers::Env, util::map};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use rocket_db_pools::Database;
use dotenv::dotenv;
use std::env;

pub mod db;
pub mod consts;
pub mod endpoints;

use endpoints::{
  api::{
    register::register as api_register,
    login::login as api_login,
    logout::logout as api_logout,
  },
  frontend::{
    index::index as fe_index,
    login::login as fe_login,
    register::register as fe_register,
    register::register_success as fe_register_success,
  }
};

#[launch]
fn rocket() -> _ {
  dotenv().ok();
  let db_url = env::var("DATABASE_URL").unwrap();
  let figment = rocket::Config::figment()
    .merge(Env::raw().only(&["PORT", "SECRET_KEY"]))
    .merge(("databases", map!["main" => map!["url" => db_url]]));
  rocket::custom(figment)
    .attach(db::MainDatabase::init())
    .attach(Template::fairing())
    .mount("/", routes![fe_index, fe_register, fe_register_success, fe_login])
    .mount("/api", routes![api_register, api_login, api_logout])
    .mount("/static", FileServer::from("./static/"))
}
