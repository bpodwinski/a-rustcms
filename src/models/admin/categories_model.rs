use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CategoryStruct {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub date_created: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CategoryNewStruct {
    pub parent_id: Option<u32>,
    pub name: String,
    pub slug: String,
    pub description: String,
}
