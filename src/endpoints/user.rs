use rocket_dyn_templates::{Template, context};
use crate::common::TemplateVars;

#[get("/user")]
pub fn user(vars: TemplateVars) -> Template {
  Template::render("user", context! { vars })
}

#[get("/user/<_id>")]
pub fn user_id(vars: TemplateVars, _id: u32) -> Template {
  Template::render("user", context! { vars })
}
