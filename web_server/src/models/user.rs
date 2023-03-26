use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct user {
    pub user_name: String,
    pub user_surname: String,
    pub user_id: String,
    pub user_password: String,
    pub user_role: String,
    pub user_status: String,
    pub user_year: i32,
}