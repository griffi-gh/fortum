 macro_rules! define_handler {
  ($fname: tt, $route: literal, $template: literal) => {
    #[get($route)]
    pub fn $fname(vars: crate::common::TemplateVars) -> (::rocket::http::ContentType, ::rocket_dyn_templates::Template) {
      (::rocket::http::ContentType::HTML, ::rocket_dyn_templates::Template::render($template, ::rocket_dyn_templates::context!{vars}))
    }
  };
}
define_handler!(sad, "/sad", "sad");
