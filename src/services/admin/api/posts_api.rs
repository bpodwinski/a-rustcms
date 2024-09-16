use reqwest::{Client, Response};

use crate::models::admin::posts_model::{
    PostNewStruct, PostRequest, PostStruct,
};

const BASE_URL: &str = "http://127.0.0.1:6988/api/v1/posts";

async fn handle_response<T>(response: Response) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    response.json::<T>().await.map_err(|e| e.to_string())
}

pub async fn get_posts() -> Result<Vec<PostStruct>, String> {
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

pub async fn get_post_by_id(post_id: u32) -> Result<PostStruct, String> {
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

pub async fn add_post(post: PostRequest) -> Result<PostStruct, String> {
    let client = Client::new();

    // Envoyer la requête POST
    let response = client
        .post(BASE_URL)
        .json(&post)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?; // Gestion des erreurs d'envoi

    let http_code = response.status().as_u16(); // Extraire le code HTTP

    // Vérifier si le code HTTP est dans la plage des succès (200–299)
    if response.status().is_success() {
        // Récupérer le corps de la réponse sous forme de `PostNewStruct`
        let mut created_post: PostStruct = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        // Ajouter le code HTTP dans la struct
        created_post.http_code = Some(http_code);

        Ok(created_post)
    } else {
        // Retourner une erreur avec le code HTTP en cas d'échec
        Err(format!("Failed to create post. HTTP Status: {}", http_code))
    }
}

pub async fn update_post(
    post_id: u32,
    post: PostNewStruct,
) -> Result<PostNewStruct, String> {
    let client = Client::new();
    let url = format!("{}/{}", BASE_URL, post_id);

    let response = client
        .put(&url)
        .json(&post)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?;

    handle_response(response).await
}
