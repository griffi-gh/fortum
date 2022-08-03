use rocket_dyn_templates::{Template, context};

#[get("/login")]
pub fn login() -> Template {
  Template::render("login", context! {})
}
