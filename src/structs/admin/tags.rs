use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TagStruct {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TagNewStruct {
    pub name: String,
    pub slug: String,
    pub description: String,
}
