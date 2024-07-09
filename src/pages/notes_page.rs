use iced::Command;

pub struct Notes {
    pub current_category: NotesCategory,
    pub show_modal: bool,
    //inputs for creating new incomes and expenses...
    name_input: String,
    price_input: String,
    category_input: String,
    date_input: String,


}

impl Notes {
    pub fn new() -> Self {
        Notes {
            current_category: NotesCategory::IncomesState,
            show_modal: false,
            name_input: String::new(),
            price_input: String::new(),
            category_input: String::new(),
            date_input: String::new(),
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

