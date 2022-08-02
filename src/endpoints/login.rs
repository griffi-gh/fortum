use rocket::form::Form;
use rocket::response::status::{BadRequest, NoContent};
use crypto::scrypt::scrypt_check;
use rocket_db_pools::Connection;
use sqlx::Row;
use crate::MainDatabase;
use crate::consts::{EMAIL_REGEX, PASSWORD_REGEX};

#[derive(FromForm)]
pub struct LoginData {
  email: String,
  password: String,
}

#[post("/login", data = "<data>")]
pub async fn login(data: Form<LoginData>, mut db: Connection<MainDatabase>) -> Result<NoContent, BadRequest<&'static str>> {
  if !EMAIL_REGEX.is_match(&data.email) {
    return Err(BadRequest(Some("Invalid email")));
  }
  if !PASSWORD_REGEX.is_match(&data.password) {
    return Err(BadRequest(Some("Invalid password")));
  }
  let hashed_password = sqlx::query("SELECT password_hash FROM users WHERE email = $1")
    .bind(&data.email)
    .fetch_optional(&mut *db).await
    .unwrap();
  if let Some(hashed_password) = hashed_password {
    let hashed_password: String = hashed_password.try_get(0).unwrap();
    //Assume that the password is in valid format
    if scrypt_check(&data.password, &hashed_password).unwrap() { 
      Ok(NoContent)
    } else {
      Err(BadRequest(Some("Incorrect password")))
    }
  } else {
    Err(BadRequest(Some("User doesn't exist")))
  }
}
