use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use sqlx;
use crate::db::MainDatabase;
use crate::common::{TemplateVars, TemplatePost};

#[get("/")]
pub async fn index(vars: TemplateVars, mut db: Connection<MainDatabase>) -> Template {
  let rows = sqlx::query("SELECT autor.username, author.profile_image, title, content, created_on, topic.topic_name, votes FROM posts ORDER BY votes DESC LIMIT 25")
    .fetch_all(&mut *db).await
    .unwrap();
  let posts: Vec<TemplatePost> = rows.into_iter().map(|row| TemplatePost::from_pg_row(row)).collect();
  Template::render("index", context! { vars, posts})
}
