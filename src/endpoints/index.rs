use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use crate::common::{TemplateVars, PostSort, SortDirection::*, PostFilter};
use crate::consts::RESULTS_PER_PAGE;

#[get("/?<page>")]
pub async fn index(vars: TemplateVars, mut db: Connection<MainDatabase>, page: Option<u32>) -> Template {
  let posts = MainDatabase::fetch_posts(
    &mut db, 
    PostSort::ByVotes(Descending), 
    PostFilter::None,
    page.unwrap_or_default(),
    RESULTS_PER_PAGE
  ).await;
  Template::render("index", context! { vars, posts })
}
