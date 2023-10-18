use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}
