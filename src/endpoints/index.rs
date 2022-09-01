use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use crate::common::template_vars::TemplateVars;
use crate::common::post::{PostSort, SortDirection, PostFilter};
use crate::consts::RESULTS_PER_PAGE;

#[get("/?<page>")]
pub async fn index(vars: TemplateVars, mut db: Connection<MainDatabase>, page: Option<u32>) -> Template {
  let posts = MainDatabase::fetch_posts(
    &mut db, 
    PostSort::ByVotes(SortDirection::Descending), 
    PostFilter::None,
    page.unwrap_or_default(),
    RESULTS_PER_PAGE
  ).await;
  let page_count = MainDatabase::count_pages(
    &mut db, 
    PostFilter::None, 
    RESULTS_PER_PAGE
  ).await;
  Template::render("index", context! { 
    vars, posts, page_count, page: page.unwrap_or_default(),
    stats: MainDatabase::get_stats(&mut db).await,
  })
}
