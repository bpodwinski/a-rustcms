use leptos::*;
use web_sys::SubmitEvent;

use crate::components::admin::header_content::HeaderContent;
use crate::services::admin::api::posts::add_post;
use crate::structs::admin::posts::PostNewStruct;
use crate::utils::add_class::add_class;

#[component]
pub fn PostNew() -> impl IntoView {
    add_class("body", "post-new");

    let (title, set_title) = create_signal(String::new());
    let (content, set_content) = create_signal(String::new());
    let (author_id, _set_author_id) = create_signal(1);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let post = PostNewStruct {
            title: title.get().clone(),
            content: content.get().clone(),
            author_id: author_id.get(),
        };

        log::info!("Submitting post: {:?}", post);

        spawn_local(async move {
            match add_post(post).await {
                Ok(returned_post) => {
                    log::info!("Post submitted successfully");
                    // Update the form fields with the returned post data
                    set_title.set(returned_post.title);
                    set_content.set(returned_post.content);
                }
                Err(e) => {
                    log::error!("Failed to submit post: {}", e);
                }
            }
        });
    };

    view! {
        <HeaderContent title="Add new post"/>

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
