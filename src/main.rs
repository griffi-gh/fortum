#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
use figment::{Figment, providers::Env, util::map};
use serde::{Serialize, Deserialize};
use rocket::fairing::AdHoc;
use rocket_dyn_templates::Template;
use rocket_db_pools::Database;
use dotenv::dotenv;
use std::env;

pub mod db;
pub mod consts;
pub mod endpoints;
pub mod common;
mod cache_file_server;

use cache_file_server::CacheFileServer;
use endpoints::{
  index::index,
  register::{register, post_register},
  login::{login, post_login},
  logout::{logout, super_logout},
  user::{user, user_self},
  submit::{submit, submit_post},
  post::post,
  vote::vote,
  dyn_profile_image::profile_image,
  delete_account::delete_account,
  update_username::update_username,
  topics::{topic, topics},
  topics_create::{topics_create, topics_create_get},
  misc::{sad, success},
  error::{default_catcher, display_error},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
  versioning: u64,
  results_per_page: u32,
  cache_length: usize,
}

#[launch]
fn rocket() -> _ {
  dotenv().ok();
  //TODO extract DATABASE_URL from figment instead
  let db_url = env::var("DATABASE_URL").unwrap(); 
  let figment = rocket::Config::figment()
    .merge(Env::raw().only(&["ADDRESS", "PORT", "SECRET_KEY"]))
    .merge(("databases", map!["main" => map!["url" => db_url]]));
  let config: Config = figment.extract().unwrap();
  rocket::custom(figment)
    .attach(db::MainDatabase::init())
    .attach(AdHoc::config::<Config>())
    .attach(Template::fairing())
    .mount("/", routes![
      index, 
      register, post_register,
      login, post_login,
      logout, super_logout,
      user, user_self,
      submit, submit_post,
      post,
      vote,
      profile_image,
      display_error,
      delete_account,
      update_username,
      topic, topics,
      topics_create, topics_create_get,
      sad, success,
    ])
    .register("/", catchers![default_catcher])
    .mount("/static", CacheFileServer::new("./static", config.cache_length))
}
