use lazy_static::lazy_static;
use regex::Regex;

//AUTH
pub const AUTH_COOKIE_NAME: &str = "auth";

//CHAT
pub const MAX_CHAT_MESSAGE_LEN: usize = 512;

//BASE64
pub(crate) const BASE64_CFG: base64::engine::general_purpose::GeneralPurpose = base64::engine::general_purpose::URL_SAFE;

//REGEX
pub const EMAIL_REGEX_STR: &str = r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";
pub const USERNAME_REGEX_STR: &str = r"^([a-zA-Z])[a-zA-Z0-9_\- ]{1,23}([a-zA-Z0-9])$";
pub const PASSWORD_REGEX_STR: &str = r".{8,}";
lazy_static! {
  pub static ref EMAIL_REGEX: Regex = Regex::new(EMAIL_REGEX_STR).unwrap();
  pub static ref USERNAME_REGEX: Regex = Regex::new(USERNAME_REGEX_STR).unwrap();
  pub static ref PASSWORD_REGEX: Regex = Regex::new(PASSWORD_REGEX_STR).unwrap();
}
