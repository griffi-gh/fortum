use regex::Regex;
lazy_static! {
  pub static ref EMAIL_REGEX: Regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
  pub static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_-]{3,15}$").unwrap();
  pub static ref PASSWORD_REGEX: Regex = Regex::new(r".{8,}").unwrap();
}
pub const AUTH_COOKIE_NAME: &str = "auth";
pub const RESULTS_PER_PAGE: u32 = 25;
