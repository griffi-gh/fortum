use super::inner_prelude::*;
use crate::common::user::User;

impl MainDatabase {
    /// Returns user id
  pub async fn get_user_id_by_token(db: &mut Connection<Self>, token: &str) -> Option<i32> {
    let result = sqlx::query("SELECT user_id FROM users WHERE token = $1")
      .bind(token)
      .fetch_optional(executor(db)).await
      .unwrap();
    result.map(|row| row.get(0))
  }

  pub async fn get_user(db: &mut Connection<Self>, user_id: i32) -> Option<User> {
    let result = sqlx::query("SELECT user_id, username, email, password_hash, created_on, last_activity, user_role, token FROM users WHERE user_id = $1")
      .bind(user_id)
      .fetch_optional(executor(db)).await
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

  pub async fn get_user_by_token(db: &mut Connection<Self>, token: &str) -> Option<User> {
    sqlx::query("SELECT user_id, username, email, password_hash, created_on, last_activity, user_role, token FROM users WHERE token = $1")
      .bind(token)
      .fetch_optional(executor(db)).await
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
}
