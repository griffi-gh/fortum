use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use crate::common::template_vars::TemplateVars;
use crate::common::post::{PostSort, SortDirection, PostFilter};
use crate::common::authentication::Authentication;
use crate::consts::RESULTS_PER_PAGE;
use super::login::rocket_uri_macro_login;

#[get("/user")]
pub async fn user_self(auth: Authentication) -> Redirect {
  Redirect::to(uri!(user(id = auth.user_id, page = Option::<u32>::None)))
}
#[get("/user", rank = 2)]
pub async fn user_self_fail() -> Redirect {
  Redirect::to(uri!(login))
}

#[get("/user/<id>?<page>")] 
pub async fn user(vars: TemplateVars, id: i32, mut db: Connection<MainDatabase>, page: Option<u32>, auth: Option<Authentication>, error: Option<FlashMessage<'_>>) -> Template {
  let self_page = auth.is_some() && (auth.unwrap().user_id == id);
  let user = match self_page {
    true => None,
    false => MainDatabase::get_user(&mut db, id).await
  };
  let posts = MainDatabase::fetch_posts(
    &mut db, 
    PostSort::ByDate(SortDirection::Descending),
    PostFilter::ByUserId(id),
    page.unwrap_or_default(),
    RESULTS_PER_PAGE
  ).await;
  let page_count = MainDatabase::count_pages(
    &mut db, 
    PostFilter::ByUserId(id),
    RESULTS_PER_PAGE
  ).await;
  Template::render("user", context! { vars, posts, user, page: page.unwrap_or_default(), page_count, self_page, error })
}
