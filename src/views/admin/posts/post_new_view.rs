use std::rc::Rc;

use leptos::*;
use strum::IntoEnumIterator;
use web_sys::SubmitEvent;

use crate::components::admin::header_content::HeaderContent;
use crate::services::admin::api::posts::add_post;
use crate::structs::admin::posts::{PostNewStruct, PostStatusEnum};
use crate::utils::add_class::add_class;

#[component]
pub fn PostNew() -> impl IntoView {
    add_class("body", "post-new");

    let (title, set_title) = create_signal(String::new());
    let (content, set_content) = create_signal(String::new());
    let (author_id, _set_author_id) = create_signal(1);
    let (status, set_status) = create_signal(PostStatusEnum::Draft);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let post = PostNewStruct {
            title: title.get().clone(),
            content: content.get().clone(),
            author_id: author_id.get(),
            status: status.get().clone(),
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

        <div class="row">

            <div class="col-md-10">
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
            </div>

            <div class="col-md-2">

                <p>Status & visibility</p>

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

        </div>
    }
}
