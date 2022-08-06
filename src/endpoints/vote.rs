use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use sqlx;
use crate::db::MainDatabase;
use crate::common::{TemplateVars, TemplatePost};

#[post("/vote")]
pub async fn vote() {
  todo!();
}
