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
    sqlx::query_as!(User, r#"
      SELECT 
        user_id, 
        username, 
        email, 
        password_hash,
        created_on,
        last_activity,
        user_role AS "user_role!: _",
        token,
        profile_image,
        banner_image
      FROM users WHERE user_id = $1"#,
      user_id
    ).fetch_optional(executor(db)).await.unwrap()
  }

  pub async fn get_user_by_token(db: &mut Connection<Self>, token: &str) -> Option<User> {
    sqlx::query_as!(User, r#"
      SELECT 
        user_id, 
        username, 
        email, 
        password_hash,
        created_on,
        last_activity,
        user_role AS "user_role!: _",
        token,
        profile_image,
        banner_image
      FROM users WHERE token = $1"#,
      token
    ).fetch_optional(executor(db)).await.unwrap()
  }
}
