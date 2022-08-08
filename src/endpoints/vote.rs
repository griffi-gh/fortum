use rocket::response::Redirect;
use rocket::form::Form;
use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use sqlx::{self, Row};
use crate::db::MainDatabase;
use crate::common::{TemplateVars, TemplatePost, Authentication};

#[derive(FromForm)]
pub struct VoteData {
  allow_toggle: bool,
  is_upvote: bool, 
  id: i32,
}

#[post("/vote/post", data = "<data>")]
pub async fn vote(data: Form<VoteData>, auth: Authentication, mut db: Connection<MainDatabase>) -> Result<String, &'static str> {
  let mut vote = if data.is_upvote { 1 } else { -1 };
  //TODO this is *trribly* inefficient
  let result = sqlx::query!(
      "SELECT vote_id, vote FROM votes WHERE user_id = $1 AND post_id = $2", 
      auth.user_id, data.id
    ).fetch_optional(&mut *db).await.unwrap();
  if let Some(result) = result {
    if data.allow_toggle {
      if data.is_upvote == result.vote {
        vote *= -1;
      }
      sqlx::query("UPDATE votes SET vote = $1 WHERE vote_id = $2")
        .bind(vote > 0)
        .bind(result.vote_id)
        .execute(&mut *db).await.unwrap();
    } else {
      return Err("You've already voted before");
    }
  } else {
    sqlx::query("INSERT INTO votes (user_id, post_id, vote) VALUES($1,$2,$3)")
      .bind(auth.user_id)
      .bind(data.id)
      .bind(vote > 0)
      .execute(&mut *db).await.unwrap();
  }
  Ok(sqlx::query("UPDATE posts SET votes = votes + $1 WHERE post_id = $2 RETURNING votes")
    .bind(vote)
    .bind(data.id)
    .fetch_one(&mut *db).await.unwrap()
    .try_get::<i64, _>(0).ok().ok_or("Post doesn't exist")?.to_string())
}
