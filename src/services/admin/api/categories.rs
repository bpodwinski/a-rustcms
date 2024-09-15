use reqwest::{Client, Response};

use crate::structs::admin::categories::{CategoryNewStruct, CategoryStruct};

const BASE_URL: &str = "http://127.0.0.1:6988/api/v1/categories";

async fn handle_response<T>(response: Response) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    response.json::<T>().await.map_err(|e| e.to_string())
}

pub async fn get_categories() -> Result<Vec<CategoryStruct>, String> {
    let client = Client::new();

    let response = client
        .get(BASE_URL)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?;

    handle_response(response).await
}

pub async fn get_category_by_id(
    post_id: u32,
) -> Result<CategoryStruct, String> {
    let client = Client::new();
    let url = format!("{}/{}", BASE_URL, post_id);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?;

    handle_response(response).await
}

pub async fn add_category(
    tag: CategoryNewStruct,
) -> Result<CategoryStruct, String> {
    let client = Client::new();

    let response = client
        .post(BASE_URL)
        .json(&tag)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?;

    handle_response(response).await
}
