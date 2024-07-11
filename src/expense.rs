use sqlx::mysql::MySqlRow;
use sqlx::Row;
use sqlx::types::time::Date;

#[derive(Debug, Clone)]
pub struct Expense {
    expenses_id: i32,
    expenses_creator: i32,
    expenses_name: String,
    expenses_price: u32,
    expenses_category: i32,
    expenses_date: Date
}

impl Expense {
    pub fn new(
        expenses_id: i32,
        expenses_creator: i32,
        expenses_name: String,
        expenses_price: u32,
        expenses_category: i32,
        expenses_date: Date
    ) -> Self {
        Self {
            expenses_id,
            expenses_creator,
            expenses_name,
            expenses_price,
            expenses_category,
            expenses_date,
        }
    }

    pub fn collect_from_query_vec(expenses_rows: Vec<MySqlRow>) -> Vec<Self> {
        expenses_rows
            .into_iter()
            .map(|expense| {
                Self::new(
                    expense.get("expense_id"),
                    expense.get("expense_creator"),
                    expense.get("expense_name"),
                    expense.get("expense_price"),
                    expense.get("expense_category"),
                    expense.get("expense_date")
                )
            }).collect()

    }
}