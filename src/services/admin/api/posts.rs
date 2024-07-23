use crate::structs::admin::posts::Post;
use reqwest::Client;

pub async fn fetch_posts() -> Result<Vec<Post>, String> {
    let client = Client::new();

    let resp = client
        .get("http://127.0.0.1:8080/api/v1/posts")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let posts = resp.json::<Vec<Post>>().await.map_err(|e| e.to_string())?;
    Ok(posts)
}
