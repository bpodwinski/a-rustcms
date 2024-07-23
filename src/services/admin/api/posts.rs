use reqwest::{Client, Response};

use crate::structs::admin::posts::{Post, PostNewStruct};

const BASE_URL: &str = "http://127.0.0.1:8080/api/v1/posts";

async fn handle_response<T>(response: Response) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    response.json::<T>().await.map_err(|e| e.to_string())
}

pub async fn get_posts() -> Result<Vec<Post>, String> {
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

pub async fn add_post(post: PostNewStruct) -> Result<PostNewStruct, String> {
    let client = Client::new();

    let response = client
        .post(BASE_URL)
        .json(&post)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?;

    handle_response(response).await
}
