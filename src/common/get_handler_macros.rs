macro_rules! define_get_handler {
  ($fname: tt, $route: literal, $template: literal) => {
    #[get($route)]
    pub fn $fname(vars: $crate::common::template_vars::TemplateVars, message: Option<::rocket::request::FlashMessage<'_>>) -> ::rocket_dyn_templates::Template {
      ::rocket_dyn_templates::Template::render($template, ::rocket_dyn_templates::context!{message, vars})
    }
  };
}
pub(crate) use define_get_handler;

macro_rules! define_get_handler_no_flash {
  ($fname: tt, $route: literal, $template: literal) => {
    #[get($route)]
    pub fn $fname(vars: $crate::common::template_vars::TemplateVars) -> ::rocket_dyn_templates::Template {
      ::rocket_dyn_templates::Template::render($template, ::rocket_dyn_templates::context!{vars})
    }
  };
}
pub(crate) use define_get_handler_no_flash;
