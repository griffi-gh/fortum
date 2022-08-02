#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
use rocket::figment::{providers::Env, util::map};
use rocket::fs::FileServer;
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;
use dotenv::dotenv;
use std::env;

pub mod consts;
pub mod endpoints;

use endpoints::{
  api::{
    register::register,
    login::login,
    logout::logout,
  },
  frontend::{
    index::index
  }
};

#[derive(Database)]
#[database("main")]
pub struct MainDatabase(sqlx::PgPool);

#[launch]
fn rocket() -> _ {
  dotenv().ok();
  let db_url = env::var("DATABASE_URL").unwrap();
  let figment = rocket::Config::figment()
    .merge(Env::raw().only(&["PORT", "SECRET_KEY"]))
    .merge(("databases", map!["main" => map!["url" => db_url]]));
  rocket::custom(figment)
    .attach(MainDatabase::init())
    .attach(Template::fairing())
    .mount("/", routes![index])
    .mount("/api", routes![register, login, logout])
    .mount("/static", FileServer::from("./static/"))
}
