
pub struct Notes {
    pub current_category: NotesCategory,
    pub show_modal: bool,
    //inputs for creating new incomes and expenses...
    pub name_input: String,
    pub price_input: String,
    pub category_input: String,
    pub date_input: String,

    //inputs for changing profile info...
    pub profile_name_input: String,
    pub profile_surname_input: String,
    pub name_input_error: Option<InputError>,
    pub surname_input_error: Option<InputError>,
}

impl Notes {
    pub fn new() -> Self {
        Notes {
            current_category: NotesCategory::MyProfile,
            show_modal: false,
            name_input: String::new(),
            price_input: String::new(),
            category_input: String::new(),
            date_input: String::new(),
            profile_name_input: String::new(),
            profile_surname_input: String::new(),
            name_input_error: None,
            surname_input_error: None
        }
    }

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NotesCategory {
    MyProfile,
    IncomesState,
    ExpensesState,
    //future categories...
}

pub enum InputError {
    WrongPassword,
    IncorrectFormat
}

