use rocket_dyn_templates::{Template, context};

#[get("/register/success")]
pub fn register_success() -> Template {
  Template::render("register", context! {
    success: true
  })
}

#[get("/register")]
pub fn register() -> Template {
  Template::render("register", context! {})
}
