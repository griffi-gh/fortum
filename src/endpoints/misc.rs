use crate::common::{define_get_handler, define_get_handler_no_flash};

define_get_handler_no_flash!(sad, "/sad", "sad");
define_get_handler!(success, "/success", "success");
