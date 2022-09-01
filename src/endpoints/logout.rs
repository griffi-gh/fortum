use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use crate::common::authentication::Authentication;
use crate::common::utils::generate_token;
use crate::consts::AUTH_COOKIE_NAME;

#[post("/super_logout")]
pub async fn super_logout(cookies: &CookieJar<'_>, mut db: Connection<MainDatabase>, auth: Authentication) -> Redirect {
  cookies.remove_private(Cookie::named(AUTH_COOKIE_NAME));
  sqlx::query("UPDATE users SET token = $1 WHERE user_id = $2")
    .bind(generate_token())
    .bind(auth.user_id)
    .execute(&mut *db).await.unwrap();
  Redirect::to(uri!("/"))
}

#[post("/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> Redirect {
  cookies.remove_private(Cookie::named(AUTH_COOKIE_NAME));
  Redirect::to(uri!("/"))
}
