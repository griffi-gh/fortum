use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Redirect, Flash};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::MainDatabase;
use crate::common::{Authentication, TemplateVars};
use crate::endpoints::login::rocket_uri_macro_login;
use crate::endpoints::post::rocket_uri_macro_post;
