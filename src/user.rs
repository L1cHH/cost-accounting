use crate::income::Income;
use crate::expense::Expense;

#[derive(Clone, Debug)]
pub struct User {
    id: i32,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub nickname: String,
    incomes: Option<Vec<Income>>,
    expenses: Option<Vec<Expense>>
}

impl User {
    pub fn new(id: i32, last_name: Option<String>, first_name: Option<String>, nickname: String) -> Self {
        User {
            id,
            last_name,
            first_name,
            nickname,
            incomes: None,
            expenses: None
        }
    }

    pub fn get_id(&self) -> i32 {
        *&self.id
    }

    pub fn add_incomes_to_user(&mut self, incomes: Vec<Income>) {
        *&mut self.incomes = Some(incomes)
    }

    pub fn add_expenses_to_user(&mut self, expenses: Vec<Expense>) {
        *&mut self.expenses = Some(expenses)
    }
}