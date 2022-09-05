use regex::Regex;
use const_random::const_random;
use const_format::formatcp;

//TODO move to config
pub const RANDOM_VERSION: &str = formatcp!("{:x}", const_random!(u128));
pub const AUTH_COOKIE_NAME: &str = "auth";
pub const RESULTS_PER_PAGE: u32 = 25;
pub const CACHE_LENGTH: usize = 1209600; //2 weeks

//REGEX
pub const EMAIL_REGEX_STR: &str = r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";
pub const USERNAME_REGEX_STR: &str = r"^([a-zA-Z])[a-zA-Z0-9_\- ]{1,23}([a-zA-Z0-9])$";
pub const PASSWORD_REGEX_STR: &str = r".{8,}";
lazy_static! {
  pub static ref EMAIL_REGEX: Regex = Regex::new(EMAIL_REGEX_STR).unwrap();
  pub static ref USERNAME_REGEX: Regex = Regex::new(USERNAME_REGEX_STR).unwrap();
  pub static ref PASSWORD_REGEX: Regex = Regex::new(PASSWORD_REGEX_STR).unwrap();
}
