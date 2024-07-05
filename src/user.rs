#[derive(Clone, Debug)]
pub struct User {
    id: i32,
    last_name: Option<String>,
    first_name: Option<String>,
    nickname: String,
}

impl User {
    pub fn new(id: i32, last_name: Option<String>, first_name: Option<String>, nickname: String) -> Self {
        User {
            id,
            last_name,
            first_name,
            nickname
        }
    }
}