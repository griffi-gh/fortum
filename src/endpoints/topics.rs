use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use crate::db::MainDatabase;
use crate::common::{TemplateVars, PostFilter, PostSort, SortDirection::*};
use crate::consts::RESULTS_PER_PAGE;

#[get("/topics")]
pub async fn topics() {
  todo!()
}

#[get("/topic/<name>?<page>")]
pub async fn topic(mut db: Connection<MainDatabase>, vars: TemplateVars, name: &str, page: Option<u32>) -> Template {
  //TODO fetch info about topic
  let posts = MainDatabase::fetch_posts(
    &mut db, 
    PostSort::ByDate(Descending),
    PostFilter::ByTopicName(name),
    page.unwrap_or_default(),
    RESULTS_PER_PAGE
  ).await;
  let page_count = MainDatabase::count_pages(
    &mut db, 
    PostFilter::ByTopicName(name),
    RESULTS_PER_PAGE
  ).await;
  Template::render("topic", context!{ vars, posts, page: page.unwrap_or_default(), page_count, _wip_name: name })
}
