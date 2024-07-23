use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub author_id: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostNewStruct {
    pub title: String,
    pub content: String,
    pub author_id: u32,
}
