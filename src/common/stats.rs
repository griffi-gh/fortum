use serde::Serialize;

#[derive(Serialize)]
pub struct Stats {
  pub posts: i32,
  pub users: i32
}
