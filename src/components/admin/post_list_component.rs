use chrono::NaiveDateTime;
use leptos::*;
use std::collections::HashSet;

use crate::models::admin::posts_model::PostStruct;

#[component]
pub fn PostList(
    posts: Signal<Vec<PostStruct>>,
    selected_posts: RwSignal<HashSet<u32>>,
) -> impl IntoView {
    let toggle_post_selection = move |post_id: u32| {
        selected_posts.update(|selected| {
            if selected.contains(&post_id) {
                selected.remove(&post_id);
            } else {
                selected.insert(post_id);
            }
        });
    };

    view! {
        <div class="card mb-3">
            <div class="card-body">
                <div class="content-list">

                    <div class="d-flex justify-content-end align-items-center w-100 my-2">
                        <div class="dropdown me-2">
                            <button
                                class="btn btn-outline-primary dropdown-toggle"
                                type="button"
                                data-bs-toggle="dropdown"
                                aria-expanded="false"
                            >
                                4/4 Columns
                            </button>
                            <ul class="dropdown-menu">
                                <li>
                                    <div class="dropdown-item">
                                        <div class="form-check">
                                            <input
                                                class="form-check-input"
                                                type="checkbox"
                                                value="title"
                                                id="checkTitle"
                                                onclick="event.stopPropagation();"
                                            />
                                            <label
                                                class="form-check-label"
                                                for="checkTitle"
                                                onclick="event.stopPropagation();"
                                            >
                                                Title
                                            </label>
                                        </div>
                                    </div>
                                </li>
                                <li>
                                    <div class="dropdown-item">
                                        <div class="form-check">
                                            <input
                                                class="form-check-input"
                                                type="checkbox"
                                                value="date_created"
                                                id="checkDateCreated"
                                                onclick="event.stopPropagation();"
                                            />
                                            <label
                                                class="form-check-label"
                                                for="checkDateCreated"
                                                onclick="event.stopPropagation();"
                                            >
                                                Date created
                                            </label>
                                        </div>
                                    </div>
                                </li>
                                <li>
                                    <div class="dropdown-item">
                                        <div class="form-check">
                                            <input
                                                class="form-check-input"
                                                type="checkbox"
                                                value="author"
                                                id="checkAuthor"
                                                onclick="event.stopPropagation();"
                                            />
                                            <label
                                                class="form-check-label"
                                                for="checkAuthor"
                                                onclick="event.stopPropagation();"
                                            >
                                                Author
                                            </label>
                                        </div>
                                    </div>
                                </li>
                                <li>
                                    <div class="dropdown-item">
                                        <div class="form-check">
                                            <input
                                                class="form-check-input"
                                                type="checkbox"
                                                value="category"
                                                id="checkCategory"
                                                onclick="event.stopPropagation();"
                                            />
                                            <label
                                                class="form-check-label"
                                                for="checkCategory"
                                                onclick="event.stopPropagation();"
                                            >
                                                Category
                                            </label>
                                        </div>
                                    </div>
                                </li>
                            </ul>
                        </div>

                        <nav aria-label="Page navigation example">
                            <ul
                                class="pagination justify-content-end"
                                style="margin-bottom: 0px !important"
                            >
                                <li class="page-item disabled">
                                    <a class="page-link">Previous</a>
                                </li>
                                <li class="page-item">
                                    <a class="page-link" href="#">
                                        1
                                    </a>
                                </li>
                                <li class="page-item">
                                    <a class="page-link" href="#">
                                        2
                                    </a>
                                </li>
                                <li class="page-item">
                                    <a class="page-link" href="#">
                                        3
                                    </a>
                                </li>
                                <li class="page-item">
                                    <a class="page-link" href="#">
                                        Next
                                    </a>
                                </li>
                            </ul>
                        </nav>
                    </div>

                    <table class="table">
                        <thead>
                            <tr>
                                <th scope="col">
                                    <input
                                        class="form-check-input"
                                        type="checkbox"
                                        aria-label="Select all posts"
                                        on:change=move |ev| {
                                            let checked = event_target_checked(&ev);
                                            if checked {
                                                let all_ids = posts
                                                    .get()
                                                    .iter()
                                                    .map(|post| post.id)
                                                    .collect::<HashSet<u32>>();
                                                selected_posts.set(all_ids);
                                            } else {
                                                selected_posts.set(HashSet::new());
                                            }
                                        }
                                    />

                                </th>
                                <th scope="col">Title</th>
                                <th scope="col">Date created</th>
                                <th scope="col">Author</th>
                                <th scope="col">#</th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || {
                                let post_list = posts.get();
                                if !post_list.is_empty() {
                                    view! {
                                        <>
                                            {post_list
                                                .into_iter()
                                                .map(|post| {
                                                    let is_checked = selected_posts.get().contains(&post.id);
                                                    view! {
                                                        <tr>
                                                            <td>
                                                                <input
                                                                    class="form-check-input"
                                                                    type="checkbox"
                                                                    on:change=move |_| toggle_post_selection(post.id)
                                                                    prop:checked=is_checked
                                                                />
                                                            </td>
                                                            <td>
                                                                <a href=format!(
                                                                    "/rs-admin/posts/{}/edit",
                                                                    post.id,
                                                                )>{&post.title}</a>
                                                                <div class="small break-word">
                                                                    {format!("Slug: {}", &post.slug)}
                                                                </div>
                                                            </td>
                                                            <td>
                                                                {post
                                                                    .date_created
                                                                    .format("%Y/%m/%d at %-I:%M %P")
                                                                    .to_string()}
                                                            </td>
                                                            <td>{post.author_id}</td>
                                                            <td>{post.id}</td>
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

                    <nav aria-label="Page navigation example">
                        <ul class="pagination justify-content-end">
                            <li class="page-item disabled">
                                <a class="page-link">Previous</a>
                            </li>
                            <li class="page-item">
                                <a class="page-link" href="#">
                                    1
                                </a>
                            </li>
                            <li class="page-item">
                                <a class="page-link" href="#">
                                    2
                                </a>
                            </li>
                            <li class="page-item">
                                <a class="page-link" href="#">
                                    3
                                </a>
                            </li>
                            <li class="page-item">
                                <a class="page-link" href="#">
                                    Next
                                </a>
                            </li>
                        </ul>
                    </nav>
                </div>
            </div>
        </div>
    }
}
