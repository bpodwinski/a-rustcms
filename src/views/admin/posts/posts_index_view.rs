use std::{collections::HashSet, sync::Arc};

use leptos::*;
use leptos_router::{use_params_map, A};

use crate::{
    components::{
        admin::{
            data_table::data_table_component::{DataTable, TableColumn},
            header_content_component::HeaderContent,
            modal_component::*,
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

    // Pagination
    let params = use_params_map();
    let page = params.with_untracked(|params| params.get("page").and_then(|p| p.parse::<u32>().ok()).unwrap_or(1));
    let current_page = create_rw_signal(page);
    let items_per_page = create_rw_signal(50);
    let (total_items_signal, set_total_items_signal) = create_signal(0);

    // Resource pour les posts paginés
    let posts = create_resource(
        move || (current_page.get(), items_per_page.get()),
        move |(page, limit)| async move { get_posts(page, limit.try_into().unwrap()).await },
    );

    let (loaded_posts, set_loaded_posts) = create_signal(Vec::<PostStruct>::new());
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
                            posts.retain(|post| !deleted_posts.ids.contains(&post.id));
                        });
                        selected_posts.set(HashSet::new());
                    }
                    Err(err) => log::error!("Error deleting posts: {}", err),
                }
            });
        }
    };

    view! {
        <HeaderContent title="Posts"/>

        <nav class="toolbar navbar sticky-top bg-body-tertiary border-bottom mb-3">
            <div class="container-fluid">
                <div class="d-flex justify-content-between align-items-center w-100 my-2">
                    <div class="d-flex">

                        <A
                            class="btn btn-primary me-2"
                            href="/rs-admin/posts/new"
                            attributes=vec![
                                ("role", Attribute::String("button".to_string().into())),
                            ]
                        >

                            <i class="bi bi-plus"></i>
                            New
                        </A>

                        {move || {
                            if !selected_posts.get().is_empty() {
                                view! {
                                    <div class="dropdown">
                                        <button
                                            class="btn btn-secondary dropdown-toggle"
                                            type="button"
                                            data-bs-toggle="dropdown"
                                            aria-expanded="false"
                                        >
                                            Actions
                                        </button>
                                        <ul class="dropdown-menu">
                                            <li>
                                                <a class="dropdown-item" href="#">
                                                    Edit status
                                                </a>
                                            </li>
                                            <li>
                                                <a class="dropdown-item" href="#">
                                                    Edit categories
                                                </a>
                                            </li>
                                            <li>
                                                <a class="dropdown-item" href="#">
                                                    Edit author
                                                </a>
                                            </li>
                                            <li>
                                                <hr class="dropdown-divider"/>
                                            </li>
                                            <li>
                                                <button
                                                    type="button"
                                                    class="dropdown-item"
                                                    data-bs-toggle="modal"
                                                    data-bs-target="#deleteModal"
                                                    on:click=move |_| is_modal_open.set(true)
                                                >
                                                    <i class="bi bi-trash-fill me-2"></i>
                                                    "Delete"
                                                </button>
                                            </li>
                                        </ul>
                                    </div>
                                }
                            } else {
                                view! {
                                    <div>
                                        <button
                                            type="button"
                                            class="btn btn-secondary dropdown-toggle"
                                            disabled
                                        >
                                            Actions
                                        </button>
                                    </div>
                                }
                            }
                        }}

                    </div>

                </div>
            </div>
        </nav>

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
                if let Some(Ok(paginated_posts)) = posts.get() {
                    set_loaded_posts.set(paginated_posts.data.clone());
                    set_total_items_signal.set(paginated_posts.total_items.try_into().unwrap());
                    let (columns, _) = create_signal(
                        vec![
                            TableColumn {
                                title: "Title",
                                value_fn: Arc::new(|post: &PostStruct| {
                                    view! {
                                        <>
                                            <a href=format!(
                                                "/rs-admin/posts/{}/edit",
                                                post.id,
                                            )>{&post.title}</a>
                                            <div class="small break-word">
                                                {format!("Slug: {}", &post.slug)}
                                            </div>
                                        </>
                                    }
                                        .into()
                                }),
                                sort_fn: Some(Arc::new(|a, b| a.title.cmp(&b.title))),
                                visible: create_rw_signal(true),
                            },
                            TableColumn {
                                title: "Date Created",
                                value_fn: Arc::new(|post: &PostStruct| {
                                    view! {
                                        <>
                                            {post.date_created.format("%Y/%m/%d").to_string()} <br/>
                                            {post.date_created.format("%-I:%M %P").to_string()}
                                        </>
                                    }
                                        .into()
                                }),
                                sort_fn: Some(Arc::new(|a, b| a.date_created.cmp(&b.date_created))),
                                visible: create_rw_signal(true),
                            },
                            TableColumn {
                                title: "Author",
                                value_fn: Arc::new(|post: &PostStruct| {
                                    view! { <>{post.author_id}</> }.into()
                                }),
                                sort_fn: Some(Arc::new(|a, b| a.author_id.cmp(&b.author_id))),
                                visible: create_rw_signal(true),
                            },
                            TableColumn {
                                title: "ID",
                                value_fn: Arc::new(|post: &PostStruct| {
                                    view! { <>{post.id}</> }.into()
                                }),
                                sort_fn: Some(Arc::new(|a, b| a.id.cmp(&b.id))),
                                visible: create_rw_signal(true),
                            },
                        ],
                    );
                    view! {
                        <DataTable
                            data=loaded_posts.into()
                            columns=columns.into()
                            selected_datas=selected_posts
                            total_items=total_items_signal.into()
                            items_per_page=items_per_page
                            page=current_page
                            on_page_change=Arc::new(move |new_page| {
                                current_page.set(new_page);
                            })

                            on_items_per_page_change=Arc::new(move |new_items_per_page| {
                                items_per_page.set(new_items_per_page.try_into().unwrap());
                            })
                        />
                    }
                } else {
                    view! { <LoadingComponent/> }
                }
            }}

        </Suspense>
    }
}
