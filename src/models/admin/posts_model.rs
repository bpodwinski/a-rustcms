use std::collections::HashSet;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize, Clone, Debug, EnumIter, Display, PartialEq, Eq, Hash)]
pub enum PostStatusEnum {
    Draft,
    Pending,
    Private,
    Scheduled,
    Published,
}

pub trait Id {
    fn id(&self) -> u32;
}

impl Id for PostStruct {
    fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PostStruct {
    pub http_code: Option<u16>,
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostsIds {
    pub ids: HashSet<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMetadata {
    pub current_page: u32,
    pub total_pages: u32,
    pub total_items: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedPosts {
    pub data: Vec<PostStruct>,
    pub current_page: u32,
    pub total_pages: u32,
    pub total_items: u32,
}
