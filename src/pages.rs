use crate::pages::notes_page::Notes;
use crate::pages::registration_login_page::{Login};
pub mod registration_login_page;
pub mod notes_page;

pub enum Page {
    LoginPage(Login),
    RegistrationPage(Login),
    NotesPage(Notes)
}



