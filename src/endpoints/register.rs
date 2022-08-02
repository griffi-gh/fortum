use rocket::form::Form;
use rocket::response::status::{BadRequest, NoContent};
use crypto::scrypt::scrypt_simple;
use rocket_db_pools::Connection;
use crate::MainDatabase;
use crate::consts::{EMAIL_REGEX, PASSWORD_REGEX, USERNAME_REGEX, SCRYPT_PARAMS};
use sqlx::{self, Row};
use rand::{thread_rng, Rng};

#[derive(FromForm)]
pub struct RegisterData {
  email: String,
  username: String,
  password: String,
}

#[post("/register", data = "<data>")]
pub async fn register(data: Form<RegisterData>, mut db: Connection<MainDatabase>) -> Result<NoContent, BadRequest<&'static str>> {
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
  let token = {
    let mut data = [0u8; 16];
    thread_rng().fill(&mut data);
    base64::encode(data)
  };
  sqlx::query("INSERT INTO users (username, email, password_hash, token) VALUES($1, $2, $3, $4);")
    .bind(&data.username)
    .bind(&data.email)
    .bind(password_hash)
    .bind(token)
    .execute(&mut *db).await
    .unwrap(); //handle error?
  Ok(NoContent)
}
