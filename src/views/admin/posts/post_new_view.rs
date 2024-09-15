use std::rc::Rc;

use chrono::NaiveDateTime;
use leptos::*;
use strum::IntoEnumIterator;
use web_sys::SubmitEvent;

use crate::components::admin::categories_component::CategoriesComponent;
use crate::components::admin::header_content_component::HeaderContent;
use crate::components::admin::publish_component::PublishComponent;
use crate::models::admin::posts_model::{
    PostNewStruct, PostRequest, PostStatusEnum,
};
use crate::services::admin::api::posts_api::add_post;
use crate::utils::add_class_util::add_class;

#[component]
pub fn AdminPostNewView() -> impl IntoView {
    add_class("body", "post-new");

    let (title, set_title) = create_signal(String::new());
    let (content, set_content) = create_signal(String::new());
    let (author_id, _set_author_id) = create_signal(1);
    let (status, set_status) = create_signal(PostStatusEnum::Draft);
    let (date_published, set_date_published) =
        create_signal(None as Option<NaiveDateTime>);
    let (categories_ids, set_categories_ids) =
        create_signal(Vec::new() as Vec<u32>);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let post = PostNewStruct {
            title: title.get().clone(),
            content: content.get().clone(),
            slug: "test-123".to_string(),
            author_id: 1,
            status: status.get().clone(),
            date_published: date_published.get().clone(),
        };

        let post_request = PostRequest {
            post,
            categories_ids: categories_ids.get().clone(),
        };

        log::info!("Submitting post: {:?}", &post_request);

        spawn_local(async move {
            match add_post(post_request).await {
                Ok(_) => log::info!("Post submitted successfully"),
                Err(e) => log::error!("Failed to submit post: {}", e),
            }
        });
    };

    view! {
        <HeaderContent title="Add new post"/>

        <div class="row">

            <div class="col-lg-8 col-xl-9">
                <form on:submit=on_submit>
                    <input type="hidden" value=author_id/>

                    <div>
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

                    <div>
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
            </div>

            <div class="col-lg-4 col-xl-3">
                <PublishComponent
                    status=status.into()
                    set_status=set_status
                    set_date_published=set_date_published
                />

                <CategoriesComponent
                    categories_ids=categories_ids.into()
                    set_categories_ids=set_categories_ids
                />
            </div>

        </div>
    }
}
