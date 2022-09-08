use super::inner_prelude::*;
use argon2::Config as ArgonConfig;
use rand::{Rng, thread_rng};
use crate::common::utils::generate_token;
use crate::consts::{EMAIL_REGEX, PASSWORD_REGEX, USERNAME_REGEX};

impl MainDatabase {
  /// Returns token
  pub async fn register(db: &mut Connection<Self>, email: &str, username: &str, password: &str) -> Result<String, &'static str> {
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
    //Check if email was used before
    let email_used: bool = sqlx::query("SELECT not COUNT(*) = 0 FROM users WHERE email = $1 LIMIT 1")
      .bind(email)
      .fetch_one(executor(db)).await
      .unwrap().get(0);
    if email_used {
      return Err("This email address is already in use");
    }
    //Register user
    let mut salt = [0u8; 16];
    thread_rng().fill(&mut salt);
    let password_hash = argon2::hash_encoded(password.as_bytes(), &salt[..], &ArgonConfig::default()).unwrap();
    let token = generate_token();
    debug_assert!(token.len() == 24, "Invalid token length");
    sqlx::query("INSERT INTO users (username, email, password_hash, token) VALUES($1, $2, $3, $4);")
      .bind(username)
      .bind(email)
      .bind(&password_hash)
      .bind(&token)
      .execute(executor(db)).await
      .unwrap();
    sqlx::query("UPDATE stats SET users = users + 1").execute(executor(db)).await.unwrap();
    Ok(token)
  }  

  /// Returns token
  pub async fn login(db: &mut Connection<Self>, email: &str, password: &str) -> Result<String, &'static str> {
    //Verify stuff
    if !EMAIL_REGEX.is_match(email) {
      return Err("Invalid email");
    }
    if !PASSWORD_REGEX.is_match(password) {
      return Err("Invalid password");
    }
    //Perform query and check if user exists
    let row = sqlx::query!("SELECT password_hash, token FROM users WHERE email = $1", email)
      .fetch_optional(executor(db)).await
      .unwrap()
      .ok_or("User doesn't exist")?;
    //Check hash (assuming it's is in valid format)
    match argon2::verify_encoded(&row.password_hash, password.as_bytes()).unwrap() { 
      true => Ok(row.token),
      false => Err("Incorrect password")
    }
  }

  pub async fn delete_account(db: &mut Connection<Self>, user_id: i32) {
    sqlx::query("DELETE FROM users WHERE user_id = $1")
      .bind(user_id)
      .execute(executor(db)).await.unwrap();
  }
}
