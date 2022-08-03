use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::response::status::{BadRequest, NoContent};
use rocket_db_pools::Connection;
use crate::db::MainDatabase;

#[derive(FromForm)]
pub struct LoginData {
  email: String,
  password: String,
}

#[post("/login", data = "<data>")]
pub async fn login(data: Form<LoginData>, db: Connection<MainDatabase>, cookies: &CookieJar<'_>) -> Result<NoContent, BadRequest<&'static str>> {
  match MainDatabase::login(db, &data.email, &data.password).await {
    Ok(token) => {
      cookies.add_private(Cookie::build("auth", token).secure(true).finish());
      Ok(NoContent)
    }
    Err(error) => Err(BadRequest(Some(error)))
  }
}
