use regex::Regex;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub const AUTH_COOKIE_NAME: &str = "auth";
pub const RESULTS_PER_PAGE: u32 = 25;

lazy_static! {
  pub static ref RANDOM: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(char::from)
    .collect();
}

//REGEX
pub const EMAIL_REGEX_STR: &str = r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";
pub const USERNAME_REGEX_STR: &str = r"^([a-zA-Z])[a-zA-Z0-9_\- ]{1,23}([a-zA-Z0-9])$";
pub const PASSWORD_REGEX_STR: &str = r".{8,}";
lazy_static! {
  pub static ref EMAIL_REGEX: Regex = Regex::new(EMAIL_REGEX_STR).unwrap();
  pub static ref USERNAME_REGEX: Regex = Regex::new(USERNAME_REGEX_STR).unwrap();
  pub static ref PASSWORD_REGEX: Regex = Regex::new(PASSWORD_REGEX_STR).unwrap();
}
