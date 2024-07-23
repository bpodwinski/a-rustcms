use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
use web_sys::SubmitEvent;

use crate::utils::add_class::add_class;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Post {
    title: String,
    content: String,
    author_id: u32,
}

#[component]
pub fn PostEdit() -> impl IntoView {
    add_class("body", "post-edit");

    let (title, set_title) = create_signal(String::new());
    let (content, set_content) = create_signal(String::new());
    let (author_id, _set_author_id) = create_signal(1);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let post = Post {
            title: title.get().clone(),
            content: content.get().clone(),
            author_id: author_id.get(),
        };

        log::info!("Submitting post: {:?}", post);

        spawn_local(async move {
            if let Err(e) = submit_post("http://127.0.0.1:8080/api/v1/posts", post).await {
                log::error!("Failed to submit post: {}", e);
            } else {
                log::info!("Post submitted successfully");
            }
        });
    };

    view! {
        <Title text="Add new post"/>
        <h1>"Add new post"</h1>

        <form on:submit=on_submit>
            <input type="hidden" value=author_id/>

            <div class="mb-3">
                <label for="post-title" class="form-label">
                    Title
                </label>
                <input
                    type="text"
                    on:input=move |ev| set_title.set(event_target_value(&ev))
                    prop:value=title
                    class="form-control"
                    id="post-title"
                />
            </div>
            <div class="mb-3">
                <label for="post-content" class="form-label">
                    Content
                </label>
                <textarea
                    on:input=move |ev| set_content.set(event_target_value(&ev))
                    prop:value=content
                    class="form-control"
                    id="post-content"
                    rows="6"
                ></textarea>
            </div>
            <button type="submit" class="btn btn-primary">
                Submit
            </button>

        </form>
    }
}

async fn submit_post(api_url: &str, post: Post) -> Result<(), String> {
    use reqwest::Client;

    let client = Client::new();

    client
        .post(api_url)
        .json(&post)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?;

    Ok(())
}
