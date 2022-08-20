use rocket::{Request, http::Status};
use rocket_dyn_templates::{Template, context};
use crate::common::TemplateVars;

#[catch(default)]
pub async fn default_catcher(status: Status, request: &Request<'_>) -> Template {
  let vars = request.guard::<TemplateVars>().await.succeeded();
  Template::render("error", context! { vars, code: status.code } )
}

#[get("/error/<code>")]
pub async fn display_error(vars: TemplateVars, code: u16) -> Template {
  Template::render("error", context! { vars, code } )
}
