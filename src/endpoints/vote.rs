use rocket::response::status::BadRequest;
use rocket::form::Form;
use rocket_db_pools::Connection;
use sqlx::{self, Row};
use crate::db::MainDatabase;
use crate::common::authentication::Authentication;

#[derive(FromForm)]
pub struct VoteData {
  allow_toggle: bool,
  is_upvote: bool, 
  id: i32,
}

#[post("/vote/post", data = "<data>")]
pub async fn vote(data: Form<VoteData>, auth: Authentication, mut db: Connection<MainDatabase>) -> Result<String, BadRequest<&'static str>> {
  // Check if post exists
  let post_exists: bool = sqlx::query("SELECT not COUNT(*) = 0 FROM posts WHERE post_id = $1")
    .bind(data.id)
    .fetch_one(&mut *db).await.unwrap()
    .get(0);
  if !post_exists {
    return Err(BadRequest(Some("Post doesn't exist")));
  }

  let mut vote_modify = if data.is_upvote { 1 } else { -1 };

  // Update/check votes table
  let result = sqlx::query!(
      "SELECT vote_id, vote FROM votes WHERE user_id = $1 AND post_id = $2", 
      auth.user_id, data.id
    ).fetch_optional(&mut *db).await.unwrap();
  if let Some(result) = result {
    if data.is_upvote == result.vote {
      // CASE 1: Vote cancelled
      if !data.allow_toggle {
        return Err(BadRequest(Some("You've already voted before")));
      }
      vote_modify *= -1;
      sqlx::query("DELETE FROM votes WHERE vote_id = $1")
        .bind(result.vote_id)
        .execute(&mut *db).await.unwrap();
    } else {
      // CASE 2: Vote updated to opposite
      vote_modify *= 2;
      sqlx::query("UPDATE votes SET vote = $1 WHERE vote_id = $2")
        .bind(data.is_upvote)
        .bind(result.vote_id)
        .execute(&mut *db).await.unwrap();
    }
  } else {
    // CASE 3: New vote
    sqlx::query("INSERT INTO votes (user_id, post_id, vote) VALUES($1,$2,$3)")
      .bind(auth.user_id)
      .bind(data.id)
      .bind(data.is_upvote)
      .execute(&mut *db).await.unwrap();
  }

  // Update post.votes
  Ok(sqlx::query("UPDATE posts SET votes = votes + $1 WHERE post_id = $2 RETURNING votes")
    .bind(vote_modify)
    .bind(data.id)
    .fetch_one(&mut *db).await.unwrap()
    .get::<i64, _>(0).to_string())
}
