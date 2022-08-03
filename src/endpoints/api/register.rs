use rocket::form::Form;
use rocket::response::status::{BadRequest, NoContent, Created};
use rocket::http::{Cookie, CookieJar};
use rocket_db_pools::Connection;
use crate::db::MainDatabase;

#[derive(FromForm)]
pub struct RegisterData {
  email: String,
  username: String,
  password: String,
}

#[post("/register", data = "<data>")]
pub async fn register(data: Form<RegisterData>, db: Connection<MainDatabase>, cookies: &CookieJar<'_>) -> Result<Created<&'static str>, BadRequest<&'static str>> {
  match MainDatabase::register(db, &data.email, &data.username, &data.password).await {
    Ok(token) => {
      cookies.add_private(Cookie::build("auth", token).secure(true).finish());
      Ok(Created::new("/register?success"))
    }
    Err(error) => Err(BadRequest(Some(error)))
  }
}
