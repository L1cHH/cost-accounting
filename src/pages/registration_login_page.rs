
pub struct Login {
    login_field: String,
    password_field: String,
    repeat_password_field: Option<String>,
    registration_error: Option<RegistrationError>,
    login_error: Option<LoginError>,
    is_registered: bool
}

impl Login {
    pub fn new_registration_page() -> Self {
        Self {
            login_field: String::new(),
            password_field: String::new(),
            repeat_password_field: Some(String::new()),
            registration_error: None,
            login_error: None,
            is_registered: false
        }
    }

    pub fn new_login_page() -> Self {
        Self {
            login_field: String::new(),
            password_field: String::new(),
            repeat_password_field: None,
            registration_error: None,
            login_error: None,
            is_registered: true
        }
    }

    pub fn get_login(&self) -> &str {
        &self.login_field
    }

    pub fn get_password(&self) -> &str {
        &self.password_field
    }

    pub fn change_password(&mut self, password: String) {
        self.password_field = password
    }

    pub fn change_login(&mut self, login: String) {
        self.login_field = login
    }

    pub fn change_repeated_password(&mut self, repeated_password: String) {
        self.repeat_password_field = Some(repeated_password)
    }

    pub fn get_repeated_password(&self) -> &str {
        self.repeat_password_field.as_ref().unwrap()
    }

    pub fn get_registration_error(&self) -> Option<&RegistrationError> {
        self.registration_error.as_ref()
    }

    pub fn set_registration_error(&mut self, error: Option<RegistrationError>) {
        self.registration_error = error
    }

    pub fn set_login_error(&mut self, error: Option<LoginError>) {
        self.login_error = error
    }

    pub fn get_login_error(&self) -> Option<&LoginError> {
        self.login_error.as_ref()
    }

    pub fn registered(&mut self) {
        self.is_registered = true
    }

    pub fn is_registered(&self) -> bool {
        if self.is_registered {true} else {false}
    }

    pub fn check_password(&mut self) -> bool {
        if is_password_relevant(&self.password_field) == false {
            self.registration_error = Some(RegistrationError::IrrelevantPassword);
            return false
        } else if is_passwords_identical(&self.get_password(), self.repeat_password_field.as_ref()) == false {
            self.registration_error = Some(RegistrationError::DifferentPasswords);
            return false
        } else {
            self.registration_error = None;
            return true
        }
    }
}

#[derive(Clone, Debug)]
pub enum RegistrationError {
    DifferentPasswords,
    IrrelevantPassword,
    UserAlreadyExists
}
#[derive(Clone, Debug)]
pub enum LoginError {
    WrongPasswordOrLogin
}

pub fn is_password_relevant(password: &str) -> bool {
    if password.chars().all(|c| (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c.is_ascii_digit()) && (password.len() > 8) {
        true
    } else {false}
}

pub fn is_passwords_identical(password: &str, repeated_password: Option<&String>) -> bool {

    match repeated_password {
        None => false,
        Some(repeated_password) => {
            if password == repeated_password {true} else {false}
        }
    }


}