use crate::components::admin::categories_component::CategoriesComponent;
use crate::components::admin::header_content_component::HeaderContent;
use crate::components::admin::publish_component::PublishComponent;
use crate::models::admin::posts_model::{
    PostNewStruct, PostRequest, PostStatusEnum,
};
use crate::services::admin::api::posts_api::add_post;
use chrono::NaiveDateTime;
use leptos::*;
use web_sys::SubmitEvent;

pub struct AdminNewContentView {
    pub name: String,
    pub title: RwSignal<String>,
    pub content: RwSignal<String>,
    pub categories_ids: RwSignal<Vec<u32>>,
    pub status: RwSignal<PostStatusEnum>,
    pub date_published: RwSignal<Option<NaiveDateTime>>,
    pub is_post: bool,
}

impl AdminNewContentView {
    // Constructeur pour un post
    pub fn new_post() -> Self {
        Self {
            name: "Add new post".to_owned(),
            title: create_rw_signal(String::new()),
            content: create_rw_signal(String::new()),
            categories_ids: create_rw_signal(Vec::new()),
            status: create_rw_signal(PostStatusEnum::Draft),
            date_published: create_rw_signal(None),
            is_post: true,
        }
    }

    // Constructeur pour une page
    pub fn new_page() -> Self {
        Self {
            name: String::new(),
            title: create_rw_signal(String::new()),
            content: create_rw_signal(String::new()),
            categories_ids: create_rw_signal(Vec::new()),
            status: create_rw_signal(PostStatusEnum::Draft),
            date_published: create_rw_signal(None),
            is_post: false,
        }
    }

    // Méthode pour soumettre un post ou une page
    pub fn submit(
        &self,
        set_notification_message: WriteSignal<String>,
        set_notification_type: WriteSignal<String>,
        set_show_toast: WriteSignal<bool>,
    ) {
        let title = self.title.get();
        let content = self.content.get();
        let categories_ids = self.categories_ids.get();
        let status = self.status.get();
        let date_published = self.date_published.get();

        let post_request = PostRequest {
            post: PostNewStruct {
                title: title.clone(),
                content: content.clone(),
                slug: title.clone(),
                author_id: 1,
                status: status.clone(),
                date_published: date_published.clone(),
            },
            categories_ids: categories_ids.clone(),
        };

        log::info!("Submitting content: {:?}", &post_request);

        spawn_local(async move {
            match add_post(post_request).await {
                Ok(created_post) => {
                    set_notification_message.set(format!(
                        "Content '{}' created successfully! (HTTP {})",
                        created_post.title,
                        created_post.http_code.unwrap_or_default()
                    ));
                    set_notification_type.set("success".to_string());
                    set_show_toast.set(true);
                }
                Err(e) => {
                    set_notification_message.set(format!("Error: {}", e));
                    set_notification_type.set("error".to_string());
                    set_show_toast.set(true);
                }
            }
        });
    }

    // Méthode pour afficher le formulaire
    pub fn render(
        &self,
        set_title: WriteSignal<String>,
        set_content: WriteSignal<String>,
        set_categories_ids: WriteSignal<Vec<u32>>,
        set_status: WriteSignal<PostStatusEnum>,
        set_date_published: WriteSignal<Option<NaiveDateTime>>,
        set_notification_message: WriteSignal<String>,
        set_notification_type: WriteSignal<String>,
        set_show_toast: WriteSignal<bool>,
    ) -> impl IntoView {
        let title = self.title.clone();
        let content = self.content.clone();
        let categories_ids = self.categories_ids.get_untracked();
        let status = self.status.get_untracked();

        view! {
            <div class="row">

                <div class="mb-3 d-flex align-items-center justify-content-start">
                    <HeaderContent title=&self.name/>
                    <button type="submit" form="post-form" class="btn btn-primary ms-3">
                        "Publish"
                    </button>
                </div>

                <div class="col-lg-8 col-xl-9">

                    <form
                        id="post-form"
                        on:submit=move |ev: SubmitEvent| {
                            ev.prevent_default();
                            let title = title.clone();
                            let content = content.clone();
                            let status = status.clone();
                            let categories_ids = categories_ids.clone();
                            let set_notification_message = set_notification_message.clone();
                            let set_notification_type = set_notification_type.clone();
                            let set_show_toast = set_show_toast.clone();
                            spawn_local(async move {
                                let post = PostNewStruct {
                                    title: title.get(),
                                    content: content.get(),
                                    slug: title.get(),
                                    author_id: 1,
                                    status,
                                    date_published: None,
                                };
                                let post_request = PostRequest {
                                    post,
                                    categories_ids: categories_ids.clone(),
                                };
                                match add_post(post_request).await {
                                    Ok(created_post) => {
                                        set_notification_message
                                            .set(
                                                format!(
                                                    "Content '{}' created successfully! (HTTP {})",
                                                    created_post.title,
                                                    created_post.http_code.unwrap_or_default(),
                                                ),
                                            );
                                        set_notification_type.set("success".to_string());
                                        set_show_toast.set(true);
                                    }
                                    Err(e) => {
                                        set_notification_message.set(format!("Error: {}", e));
                                        set_notification_type.set("error".to_string());
                                        set_show_toast.set(true);
                                    }
                                }
                            });
                        }
                    >

                        <div class="form-floating mb-3">
                            <input
                                type="text"
                                on:input=move |ev| set_title.set(event_target_value(&ev))
                                prop:value=title
                                class="form-control"
                                id="post-title"
                                placeholder="Add title"
                            />
                            <label for="post-title">"Add title"</label>
                        </div>

                        <div class="form-floating mb-3">
                            <textarea
                                on:input=move |ev| set_content.set(event_target_value(&ev))
                                prop:value=content
                                class="form-control"
                                id="post-content"
                                placeholder="Add content"
                                style="height: 200px"
                            ></textarea>
                            <label for="post-content">"Add content"</label>
                        </div>

                    </form>
                </div>

                <div class="col-lg-4 col-xl-3">
                    <PublishComponent
                        status=self.status.into()
                        set_status=set_status
                        set_date_published=set_date_published
                    />

                    <CategoriesComponent
                        categories_ids=self.categories_ids.into()
                        set_categories_ids=set_categories_ids
                    />
                </div>
            </div>
        }
    }
}
