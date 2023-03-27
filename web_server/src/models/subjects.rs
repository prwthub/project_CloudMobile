use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct subjects {
    pub python: String,
    pub java: String,
    pub matlab: String,
    pub rust: String,
}