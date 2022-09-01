use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Post {
  pub username: Option<String>,
  pub profile_image: Option<String>, 
  pub title: String, 
  pub content: Option<String>, 
  pub created_on: DateTime<Utc>, 
  pub topic_name: String, 
  pub votes: i64,
  pub user_id: Option<i32>,
  pub post_id: i32,
}

#[derive(Clone, Copy)]
pub enum SortDirection {
  Ascending, Descending  
}

#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum PostSort {
  ByDate(SortDirection),
  ByVotes(SortDirection)
}

#[derive(Clone, Copy, Default)]
#[non_exhaustive]
pub enum PostFilter<'a> {
  #[default] None,
  ByTopicId(i32),
  ByTopicName(&'a str),
  ByUserId(i32),
}
