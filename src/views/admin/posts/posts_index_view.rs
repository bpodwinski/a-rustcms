use std::collections::HashSet;

use leptos::*;

use crate::{
    components::{
        admin::{
            header_content_component::{ButtonProps, HeaderContent},
            modal_component::*,
            post_list_component::PostList,
        },
        front::loading_component::LoadingComponent,
    },
    models::admin::posts_model::{PostStruct, PostsIds},
    services::admin::api::posts_api::{delete_posts, get_posts},
    utils::add_class_util::add_class,
};

#[component]
pub fn AdminPostsView() -> impl IntoView {
    add_class("body", "posts");

    let posts = create_resource(|| (), move |_| async { get_posts().await });
    let (loaded_posts, set_loaded_posts) =
        create_signal(Vec::<PostStruct>::new());
    let selected_posts = create_rw_signal(HashSet::<u32>::new());
    let is_modal_open = create_rw_signal(false);

    // Fonction pour confirmer la suppression
    let confirm_delete = {
        let selected_posts = selected_posts.clone();
        let set_loaded_posts = set_loaded_posts.clone();
        move || {
            let posts_ids = PostsIds {
                ids: selected_posts.get_untracked().clone(),
            };
            spawn_local(async move {
                match delete_posts(posts_ids).await {
                    Ok(deleted_posts) => {
                        set_loaded_posts.update(|posts| {
                            posts.retain(|post| {
                                !deleted_posts.ids.contains(&post.id)
                            });
                        });
                        selected_posts.set(HashSet::new());
                    }
                    Err(err) => log::error!("Error deleting posts: {}", err),
                }
            });
        }
    };

    view! {
        <HeaderContent
            title="Posts"
            button=ButtonProps {
                text: "Add new post",
                url: "/rs-admin/posts/new",
            }
        />

        {move || {
            if !selected_posts.get().is_empty() {
                view! {
                    <div class="mt-3">
                        <button
                            type="button"
                            class="btn btn-danger"
                            data-bs-toggle="modal"
                            data-bs-target="#deleteModal"
                            on:click=move |_| is_modal_open.set(true)
                        >
                            "Delete posts"
                        </button>
                    </div>
                }
            } else {
                view! { <div></div> }
            }
        }}

        <Modal
            id="deleteModal".to_string()
            toggle=is_modal_open
            title="Delete posts".to_string()
            body=view! { <p>"Are you sure you want to delete the selected posts?"</p> }
            footer=Some(
                view! {
                    <button
                        type="button"
                        class="btn btn-outline-secondary"
                        data-bs-dismiss="modal"
                        on:click=move |_| is_modal_open.set(false)
                    >
                        "Cancel"
                    </button>
                    <button
                        type="button"
                        class="btn btn-danger"
                        data-bs-dismiss="modal"
                        on:click=move |_| {
                            confirm_delete();
                            is_modal_open.set(false)
                        }
                    >

                        "Delete"
                    </button>
                }
                    .into(),
            )
        />

        <Suspense fallback=move || {
            view! { <LoadingComponent/> }
        }>
            {move || {
                if let Some(Ok(posts_vec)) = posts.get() {
                    set_loaded_posts.set(posts_vec);
                    view! { <PostList posts=loaded_posts.into() selected_posts=selected_posts/> }
                } else {
                    view! { <LoadingComponent/> }
                }
            }}

        </Suspense>
    }
}
