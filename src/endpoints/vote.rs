use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use sqlx;
use crate::db::MainDatabase;
use crate::common::{TemplateVars, TemplatePost};

#[derive(FromForm)]
pub struct PostVoteData {
  allow_revoke: bool,
  is_upvote: bool, 
  id: i32,
}

#[derive(FromForm)]
pub struct CommentVoteData {
  allow_revoke: bool,
  is_upvote: bool, 
  id: i64,
}

#[post("/vote/post")]
pub async fn vote() {
  todo!();
}
