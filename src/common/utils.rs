use sqlx::Postgres;
use sqlx::pool::PoolConnection;
use rand::{Rng, thread_rng};
use rocket_db_pools::Connection;
use rocket::http::{CookieJar, Cookie, SameSite};
use crate::db::MainDatabase;
use crate::consts::AUTH_COOKIE_NAME;

pub fn div_up(a: usize, b: usize) -> usize {
  (a + (b - 1)) / b
}

//TODO allow passing any db as input
pub fn executor(db: &'_ mut Connection<MainDatabase>) -> &'_ mut PoolConnection<Postgres> {
  #[allow(clippy::explicit_auto_deref)]
  &mut *(*db)
}

//TODO accept &str
pub fn token_cookie<'a>(token: String) -> Cookie<'a> {
  Cookie::build(AUTH_COOKIE_NAME, token)
    .secure(true)
    .http_only(true)
    .same_site(SameSite::Lax)
    .permanent()
    .finish()
}

pub fn get_token(cookies: &CookieJar<'_>) -> Option<String> {
  cookies.get_private(AUTH_COOKIE_NAME).map(|x| x.value().to_owned())
}

pub fn generate_token() -> String {
  let mut data = [0u8; 16];
  thread_rng().fill(&mut data);
  base64::encode_config(data, base64::URL_SAFE)
}
