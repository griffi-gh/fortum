use super::inner_prelude::*;
use crate::common::stats::Stats;

impl MainDatabase {
  //stats
  pub async fn get_stats(db: &mut Connection<Self>) -> Stats {
    sqlx::query_as!(Stats, "SELECT * FROM stats").fetch_one(executor(db)).await.unwrap()
  }
}
