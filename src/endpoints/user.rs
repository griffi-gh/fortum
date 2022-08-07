use rocket::Either;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use crate::common::TemplateVars;

#[get("/user")]
pub async fn user_self(vars: TemplateVars) -> Template {
  Template::render("user", context! { vars, self_page: true })
}

#[get("/user/<id>")]
pub async fn user(vars: TemplateVars, id: u32, db: Connection<MainDatabase>) -> Either<Template, Redirect> {
  //Redirect to /user if id matches self
  if (id as i32) == vars.user.as_ref().map(|u| u.user_id).unwrap_or(-1) { //hacky
    return Either::Right(Redirect::to(uri!(user_self)));
  }
  let user = MainDatabase::get_user(db, id).await;
  Either::Left(Template::render("user", context! { vars, user }))
}
