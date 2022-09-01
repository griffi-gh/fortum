use sqlx::Postgres;
use sqlx::pool::PoolConnection;
use rand::{Rng, thread_rng};
use rocket_db_pools::Connection;
use rocket::http::CookieJar;
use crate::db::MainDatabase;
use crate::consts::AUTH_COOKIE_NAME;

pub fn div_up(a: usize, b: usize) -> usize {
  (a + (b - 1)) / b
}

//TODO allow passing any db as input
pub fn executor<'a>(db: &'a mut Connection<MainDatabase>) -> &'a mut PoolConnection<Postgres> {
  &mut *(*db)
}

pub fn get_token(cookies: &CookieJar<'_>) -> Option<String> {
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
