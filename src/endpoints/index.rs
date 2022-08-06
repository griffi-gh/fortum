use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use sqlx;
use crate::db::MainDatabase;
use crate::common::{TemplateVars, TemplatePost};

#[get("/")]
pub async fn index(vars: TemplateVars, mut db: Connection<MainDatabase>) -> Template {
  let rows = sqlx::query(r#"
      SELECT 
        users.username AS username, 
        users.profile_image AS profile_image,
        posts.title AS title, 
        posts.content AS content, 
        posts.created_on AS created_on, 
        topics.topic_name AS topic_name, 
        posts.votes AS votes,
        users.user_id AS user_id,
        posts.post_id as post_id
      FROM posts
      LEFT JOIN users ON users.user_id = posts.author
      INNER JOIN topics ON topics.topic_id = posts.topic
      ORDER BY votes DESC
      LIMIT 25;
    "#)
    .fetch_all(&mut *db).await
    .unwrap();
  let posts: Vec<TemplatePost> = rows.into_iter().map(|row| TemplatePost::from_pg_row(row)).collect();
  Template::render("index", context! { vars, posts })
}
