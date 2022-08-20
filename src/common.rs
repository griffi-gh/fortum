use rocket::http::CookieJar;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::Serialize;
use rocket_db_pools::Connection;
use chrono::{DateTime, Utc};
use rand::{Rng, thread_rng};
use crate::db::MainDatabase;
use crate::consts::{AUTH_COOKIE_NAME, USERNAME_REGEX_STR, PASSWORD_REGEX_STR, EMAIL_REGEX_STR};

//== UTILS =====================================================

pub fn div_up(a: usize, b: usize) -> usize {
  (a + (b - 1)) / b
}

//TODO allow passing any db as input
pub fn executor<'a>(db: &'a mut Connection<MainDatabase>) -> &'a mut sqlx::pool::PoolConnection<sqlx::Postgres> {
  &mut *(*db)
}

pub fn get_token<'a>(cookies: &CookieJar<'a>) -> Option<String> {
  match cookies.get_private(AUTH_COOKIE_NAME) {
    Some(x) => Some(x.value().to_owned()),
    None => None
  }
}

pub fn generate_token() -> String {
  let mut data = [0u8; 16];
  thread_rng().fill(&mut data);
  base64::encode_config(data, base64::URL_SAFE)
}

//== SHARED VARS ===============================================

#[derive(Serialize)]
pub struct TemplateVars {
  pub user: Option<User>,
  pub username_regex: &'static str,
  pub password_regex: &'static str,
  pub email_regex: &'static str,
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
    })
  }
}

//== AUTH ======================================================

//TODO remove lefover Option code
pub struct Authentication {
  pub token: String,
  pub user_id: i32,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authentication {
  type Error = ();
  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let jar = req.cookies();
    let token = get_token(jar);
    let user_id = if let Some(token) = token.clone() {
      let mut db = req.guard::<Connection<MainDatabase>>().await.succeeded().unwrap();
      MainDatabase::get_user_id_by_token(&mut db, &token).await
    } else { None };
    let valid = token.is_some() && user_id.is_some();
    //TODO maybe fail if no auth??
    match valid {
      true => Outcome::Success(Self {
        token: token.unwrap(),
        user_id: user_id.unwrap()
      }),
      false => Outcome::Forward(()),
    }    
  }
}

//== USER ======================================================

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

//== POSTS =====================================================

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

#[derive(Clone, Copy)]
pub enum SortDirection {
  Ascending, Descending  
}

#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum PostSort {
  ByDate(SortDirection),
  ByVotes(SortDirection)
}

#[derive(Clone, Copy, Default)]
#[non_exhaustive]
pub enum PostFilter<'a> {
  #[default] None,
  ByTopicId(i32),
  ByTopicName(&'a str),
  ByUserId(i32),
}

//== STATS =====================================================

#[derive(Serialize)]
pub struct Stats {
  pub posts: i32,
  pub users: i32
}

//==============================================================

macro_rules! define_get_handler {
  ($fname: tt, $route: literal, $template: literal) => {
    #[get($route)]
    pub fn $fname(vars: $crate::common::TemplateVars, message: Option<::rocket::request::FlashMessage<'_>>) -> ::rocket_dyn_templates::Template {
      ::rocket_dyn_templates::Template::render($template, ::rocket_dyn_templates::context!{message, vars})
    }
  };
}
pub(crate) use define_get_handler;

macro_rules! define_get_handler_no_flash {
  ($fname: tt, $route: literal, $template: literal) => {
    #[get($route)]
    pub fn $fname(vars: $crate::common::TemplateVars) -> ::rocket_dyn_templates::Template {
      ::rocket_dyn_templates::Template::render($template, ::rocket_dyn_templates::context!{vars})
    }
  };
}
pub(crate) use define_get_handler_no_flash;

//==============================================================
