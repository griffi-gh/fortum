use rocket_db_pools::{Database, Connection};
use sqlx::{self, PgPool, Row};
use crypto::scrypt::{scrypt_simple, scrypt_check};
use rand::{Rng, thread_rng};
use crate::consts::{EMAIL_REGEX, PASSWORD_REGEX, USERNAME_REGEX, SCRYPT_PARAMS};

#[derive(Database)]
#[database("main")]
pub struct MainDatabase(PgPool);
impl MainDatabase {
  /// Returns token
  pub async fn register(mut db: Connection<Self>, email: &str, username: &str, password: &str) -> Result<String, &'static str> {
    //Validate email, username and password
    if !EMAIL_REGEX.is_match(email) {
      return Err("Invalid email");
    }
    if !USERNAME_REGEX.is_match(username) {
      return Err("Invalid username");
    }
    if !PASSWORD_REGEX.is_match(password) {
      return Err("Invalid password");
    }
  
    //Check if username was used before
    //TODO this is inefficient
    let email_used: bool = sqlx::query("SELECT not COUNT(*) = 0 FROM users WHERE email = $1 LIMIT 1")
      .bind(&email)
      .fetch_one(&mut *db).await
      .unwrap().try_get(0).unwrap();
    if email_used {
      return Err("This email address is already in use");
    }
  
    //Register user
    let password_hash = scrypt_simple(&password, &SCRYPT_PARAMS).unwrap();
    let token = {
      let mut data = [0u8; 16];
      thread_rng().fill(&mut data);
      base64::encode_config(data, base64::URL_SAFE)
    };
    debug_assert!(token.len() == 24, "Invalid token length");
    sqlx::query("INSERT INTO users (username, email, password_hash, token) VALUES($1, $2, $3, $4);")
      .bind(&username)
      .bind(&email)
      .bind(&password_hash)
      .bind(&token)
      .execute(&mut *db).await
      .unwrap(); //handle error?
    Ok(token)
  }  

  /// Returns token
  pub async fn login(mut db: Connection<Self>, email: &str, password: &str) -> Result<String, &'static str> {
    if !EMAIL_REGEX.is_match(email) {
      return Err("Invalid email");
    }
    if !PASSWORD_REGEX.is_match(password) {
      return Err("Invalid password");
    }
    let row = sqlx::query("SELECT password_hash, token FROM users WHERE email = $1")
      .bind(&email)
      .fetch_optional(&mut *db).await
      .unwrap();
    if let Some(row) = row {
      let hashed_password: String = row.try_get(0).unwrap();
      let token: String = row.try_get(1).unwrap();
      //Assume that the password is in valid format
      if scrypt_check(&password, &hashed_password).unwrap() { 
        Ok(token)
      } else {
        Err("Incorrect password")
      }
    } else {
      Err("User doesn't exist")
    }
  }
}
