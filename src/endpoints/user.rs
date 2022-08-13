use rocket::Either;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use crate::common::{TemplateVars, TemplatePost, PostSort, SortDirection::*, PostFilter};
use crate::consts::RESULTS_PER_PAGE;

async fn fetch_user_posts(db: &mut Connection<MainDatabase>, id: i32, cur_page: u32) -> Vec<TemplatePost> {
  MainDatabase::fetch_posts(
    db, 
    PostSort::ByDate(Descending),
    PostFilter::ByUserId(id),
    cur_page,
    RESULTS_PER_PAGE
  ).await
}

#[get("/user?<page>")]
pub async fn user_self(vars: TemplateVars, mut db: Connection<MainDatabase>, page: Option<u32>) -> Template {
  // can't use vars.user.map(...) since async closures are not in stable yet!
  let posts = match vars.user.as_ref() {
    Some(user) => Some(fetch_user_posts(&mut db, user.user_id, page.unwrap_or_default()).await),
    None => None
  };
  Template::render("user", context! { vars, posts, page: page.unwrap_or_default(), self_page: true, })
}

#[get("/user/<id>?<page>")] 
pub async fn user(vars: TemplateVars, id: u32, mut db: Connection<MainDatabase>, page: Option<u32>) -> Either<Template, Redirect> {
  //Redirect to /user if id matches self
  if (id as i32) == vars.user.as_ref().map(|u| u.user_id).unwrap_or(-1) { //hacky
    return Either::Right(Redirect::to(uri!(user_self(page = page))));
  }
  let user = MainDatabase::get_user(&mut db, id).await;
  let posts = match user.as_ref() {
    Some(user) => Some(fetch_user_posts(&mut db, user.user_id, page.unwrap_or_default()).await),
    None => None
  };
  Either::Left(Template::render("user", context! { vars, posts, user, page: page.unwrap_or_default() }))
}
