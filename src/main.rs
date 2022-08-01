#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
use rocket::form::Form;
use rocket::response::status::{BadRequest, NoContent};
use rocket_db_pools::{Database, Connection};
use sqlx::{self, Row};
use regex::Regex;
use crypto::scrypt::{scrypt_simple, ScryptParams};

lazy_static! {
  static ref EMAIL_REGEX: Regex = Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)").unwrap();
  static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_-]{3,15}$").unwrap();
  static ref PASSWORD_REGEX: Regex = Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$").unwrap();
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
  if !EMAIL_REGEX.is_match(&data.email) {
    return Err(BadRequest(Some("Invalid email")));
  }
  if !USERNAME_REGEX.is_match(&data.username) {
    return Err(BadRequest(Some("Invalid username")));
  }
  if !PASSWORD_REGEX.is_match(&data.password) {
    return Err(BadRequest(Some("Invalid password")));
  }
  let password_hash = scrypt_simple(&data.password, &SCRYPT_PARAMS).unwrap();
  sqlx::query("INSERT VALUES($1, $2, $3) INTO users;")
    .bind(&data.username)
    .bind(&data.email)
    .bind(password_hash)
    .execute(&mut *db).await
    .unwrap(); //TODO handle me
  Ok(NoContent)
}

#[launch]
fn rocket() -> _ {
  rocket::build().attach(MainDatabase::init()).mount("/api", routes![register])
}
