use std::rc::Rc;

use chrono::NaiveDateTime;
use leptos::*;
use strum::IntoEnumIterator;

use crate::models::admin::posts_model::PostStatusEnum;

#[component]
pub fn PublishComponent(
    status: Signal<PostStatusEnum>,
    set_status: WriteSignal<PostStatusEnum>,
    set_date_published: WriteSignal<Option<NaiveDateTime>>,
) -> impl IntoView {
    view! {
        <div class="publish-component card mb-5">

            <div class="card-header">
                <h3>Publish</h3>
            </div>

            <div class="card-body">

                <div class="mb-3">
                    <h4>Status</h4>

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

                <div>
                    <h4 for="post-date-published" class="form-label">
                        Date Published
                    </h4>

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
            </div>

        </div>
    }
}
