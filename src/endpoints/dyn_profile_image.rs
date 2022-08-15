use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Cursor;
use rocket::http::ContentType;
use rocket::response::{self, Responder};
use rocket::request::Request;
use rocket::Response;

pub struct SvgResponse(String);
impl<'r, 'o: 'r> Responder<'r, 'o> for SvgResponse {
  fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'o> {
    Ok(Response::build()
      .header(ContentType::SVG)
      .raw_header("Cache-control", "max-age=2592000") // 1 month (30*24*60*60 seconds)
      .sized_body(self.0.len(), Cursor::new(self.0))
      .finalize())
  }
}

#[get("/dyn/profile_image.svg?<usr>&<id>")]
pub async fn profile_image(usr: &str, id: i32) -> SvgResponse {
  let usr_hash = if usr.trim().len() == 0 {
    0x708090 //SlateGray
  } else {
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    (hasher.finish() & 0xFFFFFF) as u32
  };
  let color = [
    ((usr_hash & 0xFF0000) >> 16) as u8,
    ((usr_hash & 0x00FF00) >> 8) as u8,
    usr_hash as u8,
  ];
  let brightness = {
    (color[0] as f32) * 0.299 + 
    (color[0] as f32) * 0.587 + 
    (color[0] as f32) * 0.114
  };
  let user_char = usr.trim_start().chars().next().unwrap_or('?');
  let svg = format!(
    "<svg version=\"1.1\" viewBox=\"0 0 64 64\" xmlns=\"http://www.w3.org/2000/svg\"><rect width=\"64\" height=\"64\" fill=\"#{color:06x}\"/><text x=\"50%\" y=\"50%\" dominant-baseline=\"{baseline}\" text-anchor=\"middle\" font-family=\"'Google Sans'\" font-size=\"40px\" fill=\"#{text_color:06x}\">{text}</text></svg>",
    color = usr_hash,
    text_color = if brightness < 186. { 0xFFFFFF } else { 0 },
    text = user_char,
    baseline = if user_char.is_lowercase() { "middle" } else { "central" } // Use `central` for upper-case and `middle` for lower-case
  );
  SvgResponse(svg)
}
