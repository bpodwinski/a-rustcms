use std::collections::HashSet;

use leptos::*;

use crate::components::admin::header_content_component::{
    ButtonProps, HeaderContent,
};
use crate::components::front::loading_component::LoadingComponent;
use crate::models::admin::posts_model::{PostStruct, PostsIds};
use crate::services::admin::api::posts_api::delete_posts;
use crate::{
    services::admin::api::posts_api::get_posts,
    utils::add_class_util::add_class,
};

// Fonction pour appeler ton service de suppression
async fn delete_selected_posts(
    selected_ids: HashSet<u32>,
    set_loaded_posts: WriteSignal<HashSet<PostStruct>>,
) -> Result<(), String> {
    // Créer l'objet PostsIds à partir des IDs sélectionnés
    let posts_ids = PostsIds {
        ids: selected_ids.into_iter().collect(),
    };

    // Appel du service delete_posts
    match delete_posts(posts_ids).await {
        Ok(deleted_posts) => {
            // Supprimer les posts localement en fonction des ids supprimés
            set_loaded_posts.update(|posts| {
                posts.retain(|post| !deleted_posts.ids.contains(&post.id));
            });
            Ok(())
        }
        Err(e) => Err(format!("Failed to delete posts: {}", e)),
    }
}

#[component]
pub fn AdminPostsView() -> impl IntoView {
    add_class("body", "posts");

    // Fetch the posts and handle the resource state
    let posts =
        create_resource(|| (), move |_| async move { get_posts().await });

    // Create a signal to store the loaded posts
    let (loaded_posts, set_loaded_posts) =
        create_signal(HashSet::<PostStruct>::new());

    let (selected_posts, set_selected_posts) =
        create_signal(HashSet::<u32>::new());

    // Signal pour indiquer si le modal est ouvert
    let (is_modal_open, set_is_modal_open) = create_signal(false);

    // Gestion de la sélection/déselection des posts
    let toggle_post_selection = move |post_id: u32| {
        let mut selected = selected_posts.get().clone();
        if selected.contains(&post_id) {
            selected.remove(&post_id);
        } else {
            selected.insert(post_id);
        }
        set_selected_posts.set(selected);
    };

    view! {
        <HeaderContent
            title="Posts"
            button=ButtonProps {
                text: "Add new post",
                url: "/rs-admin/posts/new",
            }
        />

        // Bouton "Supprimer"
        {move || {
            if !selected_posts.get().is_empty() {
                view! {
                    <div class="mt-3">
                        <button
                            type="button"
                            class="btn btn-danger"
                            data-bs-toggle="modal"
                            data-bs-target="#deleteModal"
                            on:click=move |_| {
                                set_is_modal_open.set(true);
                            }
                        >

                            "Delete posts"
                        </button>
                    </div>
                }
            } else {
                view! { <div></div> }
            }
        }}

        // Modal Bootstrap
        <div
            class=move || format!("modal fade {}", if is_modal_open.get() { "show" } else { "" })
            id="deleteModal"
            tabindex="-1"
            style=move || if is_modal_open.get() { "display: block;" } else { "display: none;" }
            aria-labelledby="deleteModalLabel"
            aria-hidden=move || (!is_modal_open.get()).to_string()
        >
            <div class="modal-dialog">
                <div class="modal-content">
                    <div class="modal-header">
                        <h1 class="modal-title fs-5" id="deleteModalLabel">
                            "Confirm Deletion"
                        </h1>
                        <button
                            type="button"
                            class="btn-close"
                            data-bs-dismiss="modal"
                            on:click=move |_| set_is_modal_open.set(false)
                            aria-label="Close"
                        ></button>
                    </div>
                    <div class="modal-body">
                        <p>"Are you sure you want to delete the selected posts?"</p>
                    </div>
                    <div class="modal-footer">
                        <button
                            type="button"
                            class="btn btn-secondary"
                            data-bs-dismiss="modal"
                            on:click=move |_| set_is_modal_open.set(false)
                        >
                            "Cancel"
                        </button>
                        <button
                            type="button"
                            class="btn btn-danger"
                            data-bs-dismiss="modal"
                            on:click=move |_| {
                                let selected_ids = selected_posts.get_untracked();
                                spawn_local(async move {
                                    if let Err(err) = delete_selected_posts(
                                            selected_ids.clone(),
                                            set_loaded_posts,
                                        )
                                        .await
                                    {
                                        log::error!("Error deleting posts: {}", err);
                                    }
                                });
                                set_is_modal_open.set(false);
                            }
                        >

                            "Delete"
                        </button>
                    </div>
                </div>
            </div>
        </div>

        <Suspense fallback=move || {
            view! { <LoadingComponent/> }
        }>
            {move || {
                if let Some(Ok(posts_vec)) = posts.get() {
                    set_loaded_posts.set(posts_vec.into_iter().collect::<HashSet<PostStruct>>());
                }
                view! {
                    <table class="table">
                        <thead>
                            <tr>
                                <th scope="col">
                                    <input
                                        class="form-check-input"
                                        type="checkbox"
                                        value=""
                                        id="flexCheckDefault"
                                        aria-label="Select all posts"
                                        on:change=move |ev| {
                                            let checked = event_target_checked(&ev);
                                            if checked {
                                                let all_ids = loaded_posts
                                                    .get()
                                                    .iter()
                                                    .map(|post| post.id)
                                                    .collect::<HashSet<u32>>();
                                                set_selected_posts.set(all_ids);
                                            } else {
                                                set_selected_posts.set(HashSet::new());
                                            }
                                        }
                                    />

                                </th>
                                <th scope="col">#</th>
                                <th scope="col">Title</th>
                                <th scope="col">Content</th>
                                <th scope="col">Author</th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || {
                                let posts_vec = loaded_posts.get();
                                if !posts_vec.is_empty() {
                                    view! {
                                        <>
                                            {posts_vec
                                                .iter()
                                                .map(|post| {
                                                    let is_checked = selected_posts.get().contains(&post.id);
                                                    let post_id = post.id;
                                                    let post_title = &post.title;
                                                    let post_content = &post.content;
                                                    let post_author_id = post.author_id;
                                                    view! {
                                                        <tr>
                                                            <td>
                                                                <input
                                                                    class="form-check-input"
                                                                    type="checkbox"
                                                                    value=""
                                                                    id="flexCheckDefault"
                                                                    on:change=move |_| toggle_post_selection(post_id)
                                                                    prop:checked=is_checked
                                                                />
                                                            </td>
                                                            <td>{post_id}</td>
                                                            <td>
                                                                <a href=format!(
                                                                    "/rs-admin/posts/{}/edit",
                                                                    post_id,
                                                                )>{post_title}</a>
                                                            </td>
                                                            <td>{post_content}</td>
                                                            <td>{post_author_id}</td>
                                                        </tr>
                                                    }
                                                })
                                                .collect::<Vec<_>>()}
                                        </>
                                    }
                                } else {
                                    view! {
                                        <>
                                            <tr>
                                                <td colspan="5">"No posts available"</td>
                                            </tr>
                                        </>
                                    }
                                }
                            }}

                        </tbody>
                    </table>
                }
            }}

        </Suspense>
    }
}
