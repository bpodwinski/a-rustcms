use leptos::*;
use leptos_router::use_params_map;

use crate::components::admin::header_content::HeaderContent;
use crate::services::admin::api::posts::get_post_by_id;
use crate::structs::admin::posts::PostStruct;
use crate::utils::add_class::add_class;

#[component]
pub fn PostEdit() -> impl IntoView {
    add_class("body", "post-edit");

    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let post_id: u32 = id().parse().unwrap_or(0);

    let (post, set_post) = create_signal(PostStruct {
        id: 0,
        title: String::new(),
        content: String::new(),
        author_id: 0,
    });

    create_effect(move |_| {
        let post_id = post_id.clone();
        spawn_local(async move {
            let post_data = get_post_by_id(post_id).await.unwrap_or(PostStruct {
                id: 0,
                title: String::new(),
                content: String::new(),
                author_id: 0,
            });
            set_post.set(post_data);
        });
    });

    view! {
        <HeaderContent title="Edit post"/>

        <form>
            <div class="mb-3">
                <label for="post-title" class="form-label">
                    Title
                </label>
                <input
                    type="text"
                    class="form-control"
                    id="post-title"
                    prop:value=move || post.with(|p| p.title.clone())
                />

            </div>
            <div class="mb-3">
                <label for="post-content" class="form-label">
                    Content
                </label>
                <textarea
                    class="form-control"
                    id="post-content"
                    rows="8"
                    prop:value=move || post.with(|p| p.content.clone())
                >
                    {post.with_untracked(|p| p.content.clone())}
                </textarea>
            </div>
            <button type="submit" class="btn btn-primary">
                Submit
            </button>
        </form>
    }
}
