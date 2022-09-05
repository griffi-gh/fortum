use rocket::State;
use rocket::request::FlashMessage;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use crate::Config;
use crate::db::MainDatabase;
use crate::common::template_vars::TemplateVars;
use crate::common::post::{PostFilter, PostSort, SortDirection};

#[get("/topics")]
pub async fn topics() {
  todo!()
}

#[get("/topic/<name>?<page>")]
pub async fn topic(
  mut db: Connection<MainDatabase>, 
  vars: TemplateVars, 
  name: &str, 
  page: Option<u32>,
  message: Option<FlashMessage<'_>>,
  config: &State<Config>
) -> Template {
  //TODO fetch info about topic
  let (posts, page_count) = MainDatabase::fetch_posts_and_count_pages(
    &mut db, 
    PostSort::ByDate(SortDirection::Descending),
    PostFilter::ByTopicName(name),
    page.unwrap_or_default(),
    config.results_per_page
  ).await;
  Template::render("topic", context!{ vars, message, posts, page: page.unwrap_or_default(), page_count, _wip_name: name })
}
