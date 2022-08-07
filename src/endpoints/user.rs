use rocket::Either;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use crate::common::{TemplateVars, TemplatePost};

async fn fetch_user_posts(mut db: Connection<MainDatabase>, id: i32) -> Vec<TemplatePost> {
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
      WHERE user_id = $1
      ORDER BY created_on DESC;
    "#)
    .bind(id)
    .fetch_all(&mut **db).await
    .unwrap();
  rows.into_iter().map(|row| TemplatePost::from_pg_row(row)).collect()
}

#[get("/user")]
pub async fn user_self(vars: TemplateVars, db: Connection<MainDatabase>) -> Template {
  let posts = match vars.user.as_ref() {
    Some(user) => Some(fetch_user_posts(db, user.user_id).await),
    None => None
  };
  Template::render("user", context! { vars, posts, self_page: true })
}

//Why the fuck do i need *two* db connections? My crappy code lol
#[get("/user/<id>")] 
pub async fn user(vars: TemplateVars, id: u32, db: Connection<MainDatabase>, db1: Connection<MainDatabase>) -> Either<Template, Redirect> {
  //Redirect to /user if id matches self
  if (id as i32) == vars.user.as_ref().map(|u| u.user_id).unwrap_or(-1) { //hacky
    return Either::Right(Redirect::to(uri!(user_self)));
  }
  let user = MainDatabase::get_user(db, id).await;
  let posts = match user.as_ref() {
    Some(user) => Some(fetch_user_posts(db1, user.user_id).await),
    None => None
  };
  Either::Left(Template::render("user", context! { vars, posts, user }))
}
