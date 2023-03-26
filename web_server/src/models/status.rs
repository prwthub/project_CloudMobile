use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct status {
    pub status: String, 
    pub message: String,
    pub sessiontime: i32,
}