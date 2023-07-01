use rocket::http::Header;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rocket_dyn_templates::{Template, context};

#[derive(Responder)]
#[response(content_type = "image/svg+xml")]
pub struct CachedSvgTemplateResponder {
  template: Template,
  header: Header<'static>,
}
impl CachedSvgTemplateResponder {
  pub fn new(template: Template) -> Self {
    Self {
      template,
      header: {
        #[cfg(not(debug_assertions))] { Header::new("Cache-control", "max-age=9999999") }
        #[cfg(debug_assertions)] { Header::new("_", "") }
      }
    }
  }
}

#[get("/dyn/profile_image.svg?<usr>&<id>")]
pub async fn profile_image(usr: Option<&str>, id: Option<i32>) -> CachedSvgTemplateResponder {
  let user_color = if usr.map(|x| x.trim().is_empty()).unwrap_or_default() {
    0x708090 //SlateGray
  } else {
    let mut hasher = DefaultHasher::new();
    match id {
      Some(id) => id.hash(&mut hasher),
      None => usr.hash(&mut hasher)
    }
    (hasher.finish() & 0xFFFFFF) as u32
  };
  let user_color_components = (
    ((user_color & 0xFF0000) >> 16) as u8,
    ((user_color & 0x00FF00) >> 8) as u8,
    user_color as u8,
  );
  let background_brightness = {
    (user_color_components.0 as f32) * 0.299 +
    (user_color_components.1 as f32) * 0.587 +
    (user_color_components.2 as f32) * 0.114
  };
  let user_char = usr.unwrap_or("?").trim_start().chars().next().unwrap_or('?');
  let text_color: u32 = if background_brightness < 186. { 0xFFFFFF } else { 0 };
  let template = Template::render("misc/profile_image", context!{
    chr: user_char,
    background_color: format!("#{:06x}", user_color),
    text_color: format!("#{:06x}", text_color),
  });
  CachedSvgTemplateResponder::new(template)
}
