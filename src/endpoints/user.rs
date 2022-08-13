use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use crate::common::{TemplateVars, TemplatePost, PostSort, SortDirection::*, PostFilter, Authentication};
use crate::consts::RESULTS_PER_PAGE;
use crate::endpoints::login::rocket_uri_macro_login;

async fn fetch_user_posts(db: &mut Connection<MainDatabase>, id: i32, cur_page: u32) -> Vec<TemplatePost> {
  MainDatabase::fetch_posts(
    db, 
    PostSort::ByDate(Descending),
    PostFilter::ByUserId(id),
    cur_page,
    RESULTS_PER_PAGE
  ).await
}

#[get("/user")]
pub async fn user_self(auth: Authentication) -> Redirect {
  let tmp: Option<u32> = None;
  Redirect::to(uri!(user(id = auth.user_id, page = tmp)))
}
#[get("/user", rank = 2)]
pub async fn user_self_fail() -> Redirect {
  let tmp: Option<&str> = None;
  Redirect::to(uri!(login(error = tmp)))
}

#[get("/user/<id>?<page>")] 
pub async fn user(vars: TemplateVars, id: i32, mut db: Connection<MainDatabase>, page: Option<u32>, auth: Option<Authentication>) -> Template {
  let self_page = auth.is_some() && (auth.unwrap().user_id == id);
  let user = match self_page {
    true => MainDatabase::get_user(&mut db, id).await,
    false => None
  };
  let posts = fetch_user_posts(&mut db, id, page.unwrap_or_default()).await;
  Template::render("user", context! { vars, posts, user, page: page.unwrap_or_default(), self_page })
}
