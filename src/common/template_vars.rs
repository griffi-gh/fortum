use serde::Serialize;
use rocket_db_pools::Connection;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use crate::db::MainDatabase;
use crate::consts::{USERNAME_REGEX_STR, PASSWORD_REGEX_STR, EMAIL_REGEX_STR, RANDOM_VERSION};
use super::user::User;
use super::utils::get_token;

#[derive(Serialize)]
pub struct TemplateVars {
  pub user: Option<User>,
  pub username_regex: &'static str,
  pub password_regex: &'static str,
  pub email_regex: &'static str,
  pub version: &'static str,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TemplateVars {
  type Error = ();
  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let jar = req.cookies();
    let token = get_token(jar);
    let mut db = req.guard::<Connection<MainDatabase>>().await.succeeded().unwrap();
    let user = if let Some(token) = token {
      MainDatabase::get_user_by_token(&mut db, &token).await
    } else { None };
    Outcome::Success(Self {
      user: user,
      username_regex: USERNAME_REGEX_STR,
      password_regex: PASSWORD_REGEX_STR,
      email_regex: EMAIL_REGEX_STR,
      version: RANDOM_VERSION,
    })
  }
}
