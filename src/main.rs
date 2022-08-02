#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
use rocket::form::Form;
use rocket::response::status::{BadRequest, NoContent};
use rocket::figment::{providers::Env, util::map};
use rocket_db_pools::{Database, Connection};
use sqlx::{self, Row};
use regex::Regex;
use crypto::scrypt::{scrypt_simple, ScryptParams};
use dotenv::dotenv;
use std::env;

lazy_static! {
  static ref EMAIL_REGEX: Regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
  static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_-]{3,15}$").unwrap();
  static ref PASSWORD_REGEX: Regex = Regex::new(r".{8,}").unwrap();
  static ref SCRYPT_PARAMS: ScryptParams = ScryptParams::new(14, 8, 1);
}

#[derive(Database)]
#[database("main")]
struct MainDatabase(sqlx::PgPool);

#[derive(FromForm)]
struct RegisterData {
  email: String,
  username: String,
  password: String,
}
#[post("/register", data = "<data>")]
async fn register(data: Form<RegisterData>, mut db: Connection<MainDatabase>) -> Result<NoContent, BadRequest<&'static str>> {
  //Validate email, username and password
  if !EMAIL_REGEX.is_match(&data.email) {
    return Err(BadRequest(Some("Invalid email")));
  }
  if !USERNAME_REGEX.is_match(&data.username) {
    return Err(BadRequest(Some("Invalid username")));
  }
  if !PASSWORD_REGEX.is_match(&data.password) {
    return Err(BadRequest(Some("Invalid password")));
  }

  //Check if username was used before
  //TODO this is inefficient
  let email_used: bool = sqlx::query("SELECT not COUNT(*) = 0 FROM users WHERE email = $1 LIMIT 1")
    .bind(&data.email)
    .fetch_one(&mut *db).await
    .unwrap().try_get(0).unwrap();
  if email_used {
    return Err(BadRequest(Some("This email address is already in use")));
  }

  //Register user
  let password_hash = scrypt_simple(&data.password, &SCRYPT_PARAMS).unwrap();
  sqlx::query("INSERT INTO users (username, email, password_hash) VALUES($1, $2, $3);")
    .bind(&data.username)
    .bind(&data.email)
    .bind(password_hash)
    .execute(&mut *db).await
    .unwrap(); //TODO handle me
  Ok(NoContent)
}

#[launch]
fn rocket() -> _ {
  dotenv().ok();
  let db_url = env::var("DATABASE_URL").unwrap();
  let figment = rocket::Config::figment()
    .merge(Env::raw().only(&["PORT"]))
    .merge(("databases", map!["main" => map!["url" => db_url]]));
  rocket::custom(figment).attach(MainDatabase::init()).mount("/api", routes![register])
}
