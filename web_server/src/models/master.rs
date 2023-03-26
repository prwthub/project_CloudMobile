use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct master {
    pub master_name: String,
    pub master_surname: String,
    pub master_role: String,
}