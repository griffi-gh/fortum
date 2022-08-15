use rocket::response::status::BadRequest;
use rocket::form::Form;
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use crate::common::Authentication;
use crate::consts::USERNAME_REGEX;

#[derive(FromForm)]
pub struct UpdateUsernameData<'a> {
  new_username: &'a str
}

#[post("/update_username", data = "<data>")]
pub async fn update_username(data: Form<UpdateUsernameData<'_>>, auth: Authentication, mut db: Connection<MainDatabase>) -> Result<(), BadRequest<&'static str>> {
  if !USERNAME_REGEX.is_match(data.new_username) {
    return Err(BadRequest(Some("Invalid username")));
  };
  sqlx::query("UPDATE users SET username = $1 WHERE user_id = $2")
    .bind(data.new_username)
    .bind(auth.user_id)
    .execute(&mut *db).await.unwrap();
  Ok(())
}
