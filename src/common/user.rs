use serde::Serialize;

#[derive(Serialize, sqlx::Type, Default)]
#[sqlx(type_name = "role_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
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
