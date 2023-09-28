use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Movie {
    pub poster: String,
    pub year: usize,
    pub name: String,
    pub path: String,
    pub category: String,
}
