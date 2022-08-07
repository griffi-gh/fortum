use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use sqlx;
use crate::db::MainDatabase;
use crate::common::{TemplateVars, TemplatePost};

#[get("/post/<id>?<success>")]
pub async fn post(id: i32, success: bool, vars: TemplateVars, mut db: Connection<MainDatabase>) -> Template {
  let post = sqlx::query_as!(TemplatePost, r#"
      SELECT 
        users.username AS "username?", 
        users.profile_image AS profile_image,
        posts.title AS title, 
        posts.content AS content, 
        posts.created_on AS created_on, 
        topics.topic_name AS topic_name, 
        posts.votes AS votes,
        users.user_id AS "user_id?",
        posts.post_id as post_id
      FROM posts
      LEFT JOIN users ON users.user_id = posts.author
      INNER JOIN topics ON topics.topic_id = posts.topic
      WHERE posts.post_id = $1
      ORDER BY votes DESC;
    "#, id)
    .fetch_optional(&mut *db).await.unwrap();
  Template::render("post", context! { vars, post, success })
}
