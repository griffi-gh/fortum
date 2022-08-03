use rocket_dyn_templates::{Template, context};
use crate::common::TemplateVars;

#[get("/")]
pub fn index(vars: TemplateVars) -> Template {
  Template::render("index", context! {
    
  })
}
