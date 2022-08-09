use rocket::http::CookieJar;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::Serialize;
use rocket_db_pools::Connection;
use chrono::{DateTime, Utc};
use crate::db::{MainDatabase, User};
use crate::consts::AUTH_COOKIE_NAME;

fn get_token<'a>(cookies: &CookieJar<'a>) -> Option<String> {
  match cookies.get_private(AUTH_COOKIE_NAME) {
    Some(x) => Some(x.value().to_owned()),
    None => None
  }
}

#[derive(Serialize, Debug)]
pub struct TemplatePost {
  pub username: Option<String>,
  pub profile_image: Option<String>, 
  pub title: String, 
  pub content: Option<String>, 
  pub created_on: DateTime<Utc>, 
  pub topic_name: String, 
  pub votes: i64,
  pub user_id: Option<i32>,
  pub post_id: i32,
}

//TODO maybe make `Authentication` return full `User` and remove TemplateVars

#[derive(Serialize)]
pub struct TemplateVars {
  pub user: Option<User>
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for TemplateVars {
  type Error = ();
  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let jar = req.cookies();
    let token = get_token(jar);
    let db = req.guard::<Connection<MainDatabase>>().await.succeeded().unwrap();
    let user = if let Some(token) = token {
      MainDatabase::get_user_by_token(db, &token).await
    } else { None };
    Outcome::Success(Self {
      user: user,
    })
  }
}

pub struct Authentication {
  pub token: Option<String>,
  pub user_id: Option<i32>,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authentication {
  type Error = ();
  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let jar = req.cookies();
    let token = get_token(jar);
    let user_id = if let Some(token) = token.clone() {
      let db = req.guard::<Connection<MainDatabase>>().await.succeeded().unwrap();
      MainDatabase::get_user_id_by_token(db, &token).await
    } else { None };
    let valid = token.is_some() && user_id.is_some();
    //TODO maybe fail if no auth??
    let slf = Self { token, user_id };
    match valid {
      true => Outcome::Success(slf),
      false => Outcome::Forward(()),
    }    
  }
}
