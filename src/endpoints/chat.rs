use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message<'a> {
  pub messsage_direction: bool,
  pub content: &'a str
}
