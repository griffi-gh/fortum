use rocket::{http::CookieJar, request::{FromRequest, Outcome}, Request, serde::Deserialize};
use rocket_db_pools::Connection;
use std::marker::PhantomData;
use crate::db::{MainDatabase, User, UserRole};
use crate::consts::AUTH_COOKIE_NAME;

fn get_token<'a>(cookies: &CookieJar<'a>) -> Option<String> {
  match cookies.get_private(AUTH_COOKIE_NAME) {
    Some(x) => Some(x.value().to_owned()),
    None => None
  }
}

pub struct TemplateVars {
  pub token: Option<String>
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for TemplateVars {
  type Error = ();
  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let jar = req.cookies();
    Outcome::Success(Self {
      token: get_token(jar)
    })
  }
}

pub struct Authentication {
  pub valid: bool,
  pub token: Option<String>,
  pub user_id: Option<u32>,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authentication {
  type Error = ();
  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let jar = req.cookies();
    let token = get_token(jar);
    let user_id = if let Some(token) = token.clone() {
      let db = req.guard::<Connection<MainDatabase>>().await.succeeded().unwrap();
      MainDatabase::get_token_user(db, &token).await
    } else { None };
    let valid = token.is_some() && user_id.is_some();
    Outcome::Success(Self {
      valid, token, user_id
    })
  }
}
