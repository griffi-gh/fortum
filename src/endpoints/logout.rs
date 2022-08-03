use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use crate::consts::AUTH_COOKIE_NAME;

#[post("/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> Redirect {
  cookies.remove_private(Cookie::named(AUTH_COOKIE_NAME));
  Redirect::to(uri!("/"))
}
