use std::rc::Rc;

use chrono::NaiveDateTime;
use leptos::*;
use strum::IntoEnumIterator;
use web_sys::SubmitEvent;

use crate::components::admin::header_content::HeaderContent;
use crate::services::admin::api::posts::add_post;
use crate::structs::admin::posts::{
    PostNewStruct, PostRequest, PostStatusEnum,
};
use crate::utils::add_class::add_class;

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
        create_signal(Vec::new() as Vec<i32>);

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
                <div class="mb-3">
                    <h3>Status & visibility</h3>

                    {PostStatusEnum::iter()
                        .map(|status_option| {
                            let status_option_str = format!("{:?}", status_option);
                            let status_option = Rc::new(status_option);
                            let status_option_clone = Rc::clone(&status_option);
                            let status_option_clone2 = Rc::clone(&status_option);
                            view! {
                                <div class="form-check">
                                    <input
                                        class="form-check-input"
                                        type="radio"
                                        name="post-status"
                                        id=status_option_str.clone()
                                        on:change=move |_| {
                                            set_status.set((*status_option_clone).clone())
                                        }

                                        prop:checked=move || *status_option_clone2 == status.get()
                                    />
                                    <label class="form-check-label" for=status_option_str.clone()>
                                        {status_option_str}
                                    </label>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>

                <div class="mb-3">
                    <h3 for="post-date-published" class="form-label">
                        Date Published
                    </h3>

                    <input
                        type="datetime-local"
                        on:input=move |ev| {
                            let date = event_target_value(&ev);
                            let parsed_date = NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M")
                                .ok();
                            set_date_published.set(parsed_date);
                        }

                        class="form-control"
                        id="post-date-published"
                    />
                </div>

                <div class="mb-3">
                    <h3 for="post-categories" class="form-label">
                        Categories
                    </h3>

                    <input
                        type="text"
                        on:input=move |ev| {
                            let categories_input = event_target_value(&ev);
                            let ids: Vec<i32> = categories_input
                                .split(',')
                                .filter_map(|s| s.trim().parse::<i32>().ok())
                                .collect();
                            set_categories_ids.set(ids);
                        }

                        class="form-control"
                        id="post-categories"
                        placeholder="Enter category IDs separated by commas (e.g., 4, 6)"
                    />
                </div>

            </div>

        </div>
    }
}
