use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket_db_pools::Connection;
use super::utils::get_token;
use crate::db::MainDatabase;

pub struct Authentication {
  pub token: String,
  pub user_id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authentication {
  type Error = ();
  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let jar = req.cookies();
    let token = get_token(jar);
    let user_id = if let Some(token) = token.clone() {
      let mut db = req.guard::<Connection<MainDatabase>>().await.succeeded().unwrap();
      MainDatabase::get_user_id_by_token(&mut db, &token).await
    } else { None };
    let valid = token.is_some() && user_id.is_some();
    //TODO maybe fail if no auth??
    match valid {
      true => Outcome::Success(Self {
        token: token.unwrap(),
        user_id: user_id.unwrap()
      }),
      false => Outcome::Forward(()),
    }    
  }
}

//TODO authorization
