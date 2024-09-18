use leptos::*;
use std::collections::HashSet;

use crate::models::admin::posts_model::PostStruct;

#[derive(Clone, Copy, PartialEq, Eq)]
enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SortColumn {
    Title,
    DateCreated,
    Author,
    ID,
}

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

    // Signal to track the current sorting column and order
    let sort_column = create_rw_signal(SortColumn::ID);
    let sort_order = create_rw_signal(SortOrder::Descending);

    let sort_posts = move |posts: &mut Vec<PostStruct>| {
        let column = sort_column.get();
        let order = sort_order.get();

        match column {
            SortColumn::Title => {
                if order == SortOrder::Ascending {
                    posts.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
                } else {
                    posts.sort_by(|a, b| b.title.to_lowercase().cmp(&a.title.to_lowercase()));
                }
            }
            SortColumn::DateCreated => {
                if order == SortOrder::Ascending {
                    posts.sort_by(|a, b| a.date_created.cmp(&b.date_created));
                } else {
                    posts.sort_by(|a, b| b.date_created.cmp(&a.date_created));
                }
            }
            SortColumn::Author => {
                if order == SortOrder::Ascending {
                    posts.sort_by(|a, b| a.author_id.cmp(&b.author_id));
                } else {
                    posts.sort_by(|a, b| b.author_id.cmp(&a.author_id));
                }
            }
            SortColumn::ID => {
                if order == SortOrder::Ascending {
                    posts.sort_by(|a, b| a.id.cmp(&b.id));
                } else {
                    posts.sort_by(|a, b| b.id.cmp(&a.id));
                }
            }
        }
    };

    let toggle_sort = move |column: SortColumn| {
        if sort_column.get() == column {
            // Toggle the sort order if the column is already selected
            sort_order.update(|order| {
                if *order == SortOrder::Ascending {
                    *order = SortOrder::Descending;
                } else {
                    *order = SortOrder::Ascending;
                }
            });
        } else {
            // Set the new column and reset to ascending order
            sort_column.set(column);
            sort_order.set(SortOrder::Ascending);
        }
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
                                <th scope="col">
                                    <button
                                        class="btn btn-dark"
                                        on:click=move |_| toggle_sort(SortColumn::Title)
                                    >
                                        Title
                                        {move || {
                                            if sort_column.get() == SortColumn::Title {
                                                if sort_order.get() == SortOrder::Ascending {
                                                    view! { <i class="bi bi-sort-down-alt ms-1"></i> }
                                                } else {
                                                    view! { <i class="bi bi-sort-up ms-1"></i> }
                                                }
                                            } else {
                                                view! { <i class="bi bi-sort-up ms-1"></i> }
                                            }
                                        }}

                                    </button>
                                </th>
                                <th scope="col">
                                    <button
                                        class="btn btn-dark"
                                        on:click=move |_| toggle_sort(SortColumn::DateCreated)
                                    >
                                        Date created
                                        {move || {
                                            if sort_column.get() == SortColumn::DateCreated {
                                                if sort_order.get() == SortOrder::Ascending {
                                                    view! { <i class="bi bi-sort-down-alt ms-1"></i> }
                                                } else {
                                                    view! { <i class="bi bi-sort-up ms-1"></i> }
                                                }
                                            } else {
                                                view! { <i class="bi bi-sort-up ms-1"></i> }
                                            }
                                        }}

                                    </button>
                                </th>
                                <th scope="col">
                                    <button
                                        class="btn btn-dark"
                                        on:click=move |_| toggle_sort(SortColumn::Author)
                                    >
                                        Author
                                        {move || {
                                            if sort_column.get() == SortColumn::Author {
                                                if sort_order.get() == SortOrder::Ascending {
                                                    view! { <i class="bi bi-sort-down-alt ms-1"></i> }
                                                } else {
                                                    view! { <i class="bi bi-sort-up ms-1"></i> }
                                                }
                                            } else {
                                                view! { <i class="bi bi-sort-up ms-1"></i> }
                                            }
                                        }}

                                    </button>
                                </th>
                                <th scope="col">
                                    <button
                                        type="button"
                                        class="btn btn-dark"
                                        on:click=move |_| toggle_sort(SortColumn::ID)
                                    >
                                        ID

                                        {move || {
                                            if sort_column.get() == SortColumn::ID {
                                                if sort_order.get() == SortOrder::Ascending {
                                                    view! { <i class="bi bi-sort-down-alt ms-1"></i> }
                                                } else {
                                                    view! { <i class="bi bi-sort-up ms-1"></i> }
                                                }
                                            } else {
                                                view! { <i class="bi bi-sort-up ms-1"></i> }
                                            }
                                        }}

                                    </button>
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || {
                                let mut post_list = posts.get();
                                sort_posts(&mut post_list);
                                if !post_list.is_empty() {
                                    view! {
                                        <>
                                            {post_list
                                                .into_iter()
                                                .map(|post| {
                                                    let is_checked = selected_posts.get().contains(&post.id);
                                                    let row_class = if is_checked {
                                                        "table-active"
                                                    } else {
                                                        ""
                                                    };
                                                    view! {
                                                        <tr class=row_class>
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
                                        // Sort posts based on the current sorting state

                                        <>
                                            <tr>
                                                <td colspan="5">"No posts available"</td>
                                            </tr>
                                        </>
                                    }
                                }
                            }}

                        </tbody>
                        <caption>34 items</caption>
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
