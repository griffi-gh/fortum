use rocket_dyn_templates::{Template, context};

#[get("/register")]
pub fn register() -> Template {
  Template::render("register", context! {})
}
