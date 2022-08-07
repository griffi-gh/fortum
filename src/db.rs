use rocket::serde::Serialize;
use rocket_db_pools::{Database, Connection};
use sqlx::{self, PgPool, Row};
use argon2::{self, Config as ArgonConfig};
use rand::{Rng, thread_rng};
use crate::consts::{EMAIL_REGEX, PASSWORD_REGEX, USERNAME_REGEX};

#[derive(Serialize, sqlx::Type, Default)]
#[sqlx(type_name = "role_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
  Banned,
  Unverified,
  #[default]
  User,
  Moderator,
  Admin
}

#[derive(Serialize)]
pub struct User {
  pub user_id: i32,
  pub username: String,
  pub email: String,
  pub password_hash: String,
  pub created_on: chrono::DateTime<chrono::Utc>,
  pub last_activity: chrono::DateTime<chrono::Utc>,
  pub user_role: UserRole,
  pub token: String,
}

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
    //Check if email was used before
    let email_used: bool = sqlx::query("SELECT not COUNT(*) = 0 FROM users WHERE email = $1 LIMIT 1")
      .bind(&email)
      .fetch_one(&mut *db).await
      .unwrap().get(0);
    if email_used {
      return Err("This email address is already in use");
    }
    //Register user
    let mut salt = [0u8; 16];
    thread_rng().fill(&mut salt);
    let password_hash = argon2::hash_encoded(password.as_bytes(), &salt[..], &ArgonConfig::default()).unwrap();
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
    //Verify stuff
    if !EMAIL_REGEX.is_match(email) {
      return Err("Invalid email");
    }
    if !PASSWORD_REGEX.is_match(password) {
      return Err("Invalid password");
    }
    //Perform query and check if user exists
    let row = sqlx::query!("SELECT password_hash, token FROM users WHERE email = $1", email)
      .fetch_optional(&mut *db).await
      .unwrap()
      .ok_or("User doesn't exist")?;
    //Check hash (assuming it's is in valid format)
    match argon2::verify_encoded(&row.password_hash, password.as_bytes()).unwrap() { 
      true => Ok(row.token),
      false => Err("Incorrect password")
    }
  }

  /// Returns user id
  pub async fn get_user_id_by_token(mut db: Connection<Self>, token: &str) -> Option<i32> {
    let result = sqlx::query("SELECT user_id FROM users WHERE token = $1")
      .bind(token)
      .fetch_optional(&mut *db).await
      .unwrap();
    result.map(|row| row.get(0))
  }

  pub async fn get_user(mut db: Connection<Self>, user_id: u32) -> Option<User> {
    let result = sqlx::query("SELECT user_id, username, email, password_hash, created_on, last_activity, user_role, token FROM users WHERE user_id = $1")
      .bind(user_id as i32)
      .fetch_optional(&mut *db).await
      .unwrap();
    result.map(|row| User {
      user_id: row.get(0),
      username: row.get(1),
      email: row.get(2),
      password_hash: row.get(3),
      created_on: row.get(4),
      last_activity: row.get(5),
      user_role: row.get(6),
      token: row.get(7),
    })
  }

  pub async fn get_user_by_token(mut db: Connection<Self>, token: &str) -> Option<User> {
    sqlx::query("SELECT user_id, username, email, password_hash, created_on, last_activity, user_role, token FROM users WHERE token = $1")
      .bind(token)
      .fetch_optional(&mut *db).await
      .unwrap()
      .map(|row| User {
        user_id: row.get(0),
        username: row.get(1),
        email: row.get(2),
        password_hash: row.get(3),
        created_on: row.get(4),
        last_activity: row.get(5),
        user_role: row.get(6),
        token: row.get(7),
      })
  }

  // pub async fn fetch_posts(mut db: Connection<Self>, topic: Option<&str>, ) {
  //   todo!();
  // }

  // Assumes that user exists!
  pub async fn submit_post(mut db: Connection<Self>, author: Option<i32>, topic_id: i32, title: &str, body: Option<&str>) -> Result<i32, &'static str> {
    if title.trim().is_empty() {
      return Err("Empty post title"); 
    }
    let mut body = body;
    if let Some(body_inner) = body {
      if body_inner.trim().is_empty() {
        body = None;
      }
    }
    let topic_exists = sqlx::query("SELECT COUNT(*) FROM topics WHERE topic_id = $1 LIMIT 1")
      .bind(topic_id)
      .fetch_optional(&mut *db).await
      .unwrap()
      .map(|x| {
        let len: i64 = x.get(0);
        len > 0
      })
      .unwrap_or_default();
    if !topic_exists {
      return Err("Topic doensn't exist");
    }
    let post_id = sqlx::query("INSERT INTO posts (author, topic, title, content) VALUES($1, $2, $3, $4) RETURNING post_id")
      .bind(author)
      .bind(topic_id)
      .bind(title)
      .bind(body)
      .fetch_one(&mut *db).await
      .unwrap().get(0);
    Ok(post_id)
  }
}
