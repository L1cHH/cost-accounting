
use crate::pages::registration_login_page::{Login};
pub mod registration_login_page;

pub enum Page {
    LoginPage(Login),
    RegistrationPage(Login)
}



