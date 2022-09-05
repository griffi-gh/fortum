use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use rocket::State;
use crate::Config;
use crate::db::MainDatabase;
use crate::common::template_vars::TemplateVars;
use crate::common::post::{PostSort, SortDirection, PostFilter};

#[get("/?<page>")]
pub async fn index(vars: TemplateVars, mut db: Connection<MainDatabase>, page: Option<u32>, config: &State<Config>) -> Template {
  let (posts, page_count) = MainDatabase::fetch_posts_and_count_pages(
    &mut db, 
    PostSort::ByVotes(SortDirection::Descending), 
    PostFilter::None,
    page.unwrap_or_default(),
    config.results_per_page
  ).await;
  Template::render("index", context! { 
    vars, posts, page_count, page: page.unwrap_or_default(),
    stats: MainDatabase::get_stats(&mut db).await,
  })
}
