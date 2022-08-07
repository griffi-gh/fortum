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
pub mod common;

use endpoints::{
  index::index,
  register::{register, post_register, register_success},
  login::{login, post_login},
  logout::logout,
  user::{user, user_self},
  submit::{submit, submit_post, submit_post_error},
  post::post,
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
    .mount("/", routes![
      index, 
      register, post_register, register_success,
      login, post_login,
      logout, 
      user, user_self,
      submit, submit_post, submit_post_error,
      post
    ])
    .mount("/static", FileServer::from("./static/"))
}
