use rocket::State;
use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use crate::Config;
use crate::db::MainDatabase;
use crate::common::template_vars::TemplateVars;
use crate::common::post::{PostSort, SortDirection, PostFilter};
use crate::common::authentication::Authentication;

#[get("/user")]
pub async fn user_self(auth: Authentication) -> Redirect {
  Redirect::to(uri!(user(id = auth.user_id, page = Option::<u32>::None)))
}

#[get("/user/<id>?<page>")] 
pub async fn user(
  vars: TemplateVars, 
  id: i32, 
  mut db: Connection<MainDatabase>, 
  page: Option<u32>,
  auth: Option<Authentication>,
  error: Option<FlashMessage<'_>>,
  config: &State<Config>
) -> Template {
  let is_self_page = auth.is_some() && (auth.unwrap().user_id == id);
  let user = match is_self_page {
    true => None,
    false => MainDatabase::get_user(&mut db, id).await
  };
  let (posts, page_count) = MainDatabase::fetch_posts_and_count_pages(
    &mut db, 
    PostSort::ByDate(SortDirection::Descending),
    PostFilter::ByUserId(id),
    page.unwrap_or_default(),
    config.results_per_page
  ).await;
  Template::render("user", context! { 
    vars, 
    posts, 
    user, 
    page: page.unwrap_or_default(), 
    page_count, 
    self_page: is_self_page, 
    error
  })
}
