use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(
    Serialize, Deserialize, Clone, Debug, EnumIter, Display, PartialEq,
)]
pub enum PostStatusEnum {
    Draft,
    Pending,
    Private,
    Scheduled,
    Published,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostStruct {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub author_id: u32,
    pub status: PostStatusEnum,
    pub date_published: Option<NaiveDateTime>,
    pub date_created: NaiveDateTime,
    pub categories: Vec<Category>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostNewStruct {
    pub title: String,
    pub content: String,
    pub slug: String,
    pub author_id: u32,
    pub status: PostStatusEnum,
    pub date_published: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostRequest {
    pub post: PostNewStruct,
    pub categories_ids: Vec<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub description: String,
}
