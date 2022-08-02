use rocket::http::{Cookie, CookieJar};
use rocket::response::status::NoContent;
use crate::consts::AUTH_COOKIE_NAME;

#[post("/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> NoContent {
  cookies.remove_private(Cookie::named(AUTH_COOKIE_NAME));
  NoContent
}
