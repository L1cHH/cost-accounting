use sqlx::mysql::MySqlRow;
use sqlx::Row;
use sqlx::types::time::Date;

#[derive(Clone, Debug)]
pub struct Income {
    incomes_id: i32,
    incomes_creator: i32,
    incomes_name: String,
    incomes_price: u32,
    incomes_category: i32,
    incomes_date: Date
}

impl Income {
    pub fn new(
        incomes_id: i32,
        incomes_creator: i32,
        incomes_name: String,
        incomes_price: u32,
        incomes_category: i32,
        incomes_date: Date
    ) -> Self {
        Self {
            incomes_id,
            incomes_creator,
            incomes_name,
            incomes_price,
            incomes_category,
            incomes_date,
        }
    }

    pub fn collect_from_query_vec(incomes_rows: Vec<MySqlRow>) -> Vec<Self> {
        incomes_rows
            .into_iter()
            .map(|income| {
                Self::new(
                    income.get("incomes_id"),
                    income.get("incomes_creator"),
                    income.get("incomes_name"),
                    income.get("incomes_price"),
                    income.get("incomes_category"),
                    income.get("incomes_date")
                )
            }).collect()

    }
}