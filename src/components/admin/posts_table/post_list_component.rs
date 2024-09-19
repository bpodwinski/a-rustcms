use ev::Event;
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

    // Signaux pour suivre la visibilité des colonnes
    let (show_date_created, set_show_date_created) = create_signal(true);
    let (show_author, set_show_author) = create_signal(true);
    let (show_id, set_show_id) = create_signal(true);

    let total_columns = 4;
    // Calculer le nombre de colonnes actuellement visibles
    let visible_columns = move || {
        let mut count = 1; // Le titre est toujours visible
        if show_date_created.get() {
            count += 1;
        }
        if show_author.get() {
            count += 1;
        }
        if show_id.get() {
            count += 1;
        }
        count
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

    // Gérer la sélection du tri via le `select`
    let handle_sort_change = move |ev: Event| {
        let value = event_target_value(&ev);
        let (column, order) = match value.as_str() {
            "TitleAscending" => (SortColumn::Title, SortOrder::Ascending),
            "TitleDescending" => (SortColumn::Title, SortOrder::Descending),
            "IdAscending" => (SortColumn::ID, SortOrder::Ascending),
            "IdDescending" => (SortColumn::ID, SortOrder::Descending),
            "DateCreatedAscending" => (SortColumn::DateCreated, SortOrder::Ascending),
            "DateCreatedDescending" => (SortColumn::DateCreated, SortOrder::Descending),
            "AuthorAscending" => (SortColumn::Author, SortOrder::Ascending),
            "AuthorDescending" => (SortColumn::Author, SortOrder::Descending),
            _ => (SortColumn::ID, SortOrder::Descending),
        };
        sort_column.set(column);
        sort_order.set(order);
    };

    // Générer la valeur de tri actuelle pour le select
    let current_sort_value = move || match sort_column.get() {
        SortColumn::Title => match sort_order.get() {
            SortOrder::Ascending => "TitleAscending".to_string(),
            SortOrder::Descending => "TitleDescending".to_string(),
        },
        SortColumn::ID => match sort_order.get() {
            SortOrder::Ascending => "IdAscending".to_string(),
            SortOrder::Descending => "IdDescending".to_string(),
        },
        SortColumn::DateCreated => match sort_order.get() {
            SortOrder::Ascending => "DateCreatedAscending".to_string(),
            SortOrder::Descending => "DateCreatedDescending".to_string(),
        },
        SortColumn::Author => match sort_order.get() {
            SortOrder::Ascending => "AuthorAscending".to_string(),
            SortOrder::Descending => "AuthorDescending".to_string(),
        },
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

                    <select
                        class="form-select me-2"
                        aria-label="Sort Table By"
                        style="width: fit-content;"
                        on:change=move |ev| handle_sort_change(ev)
                    >
                        <option
                            value="TitleAscending"
                            selected=move || current_sort_value() == "TitleAscending"
                        >
                            Title ascending
                        </option>
                        <option
                            value="TitleDescending"
                            selected=move || current_sort_value() == "TitleDescending"
                        >
                            Title descending
                        </option>
                        <option
                            value="DateCreatedAscending"
                            selected=move || current_sort_value() == "DateCreatedAscending"
                        >
                            Date created ascending
                        </option>
                        <option
                            value="DateCreatedDescending"
                            selected=move || current_sort_value() == "DateCreatedDescending"
                        >
                            Date created descending
                        </option>
                        <option
                            value="AuthorAscending"
                            selected=move || current_sort_value() == "AuthorAscending"
                        >
                            Author ascending
                        </option>
                        <option
                            value="AuthorDescending"
                            selected=move || current_sort_value() == "AuthorDescending"
                        >
                            Author descending
                        </option>
                        <option
                            value="IdAscending"
                            selected=move || current_sort_value() == "IdAscending"
                        >
                            ID ascending
                        </option>
                        <option
                            value="IdDescending"
                            selected=move || current_sort_value() == "IdDescending"
                        >
                            ID descending
                        </option>
                    </select>

                    <div class="d-flex justify-content-end align-items-center w-100 my-2">

                        <div class="dropdown me-2">
                            <button
                                class="btn btn-outline-primary dropdown-toggle"
                                type="button"
                                data-bs-toggle="dropdown"
                                aria-expanded="false"
                            >
                                {move || format!("{}/{} Columns", visible_columns(), total_columns)}
                            </button>
                            <ul class="dropdown-menu">
                                <li>
                                    <div class="dropdown-item">
                                        <div class="form-check" onclick="event.stopPropagation();">
                                            <input
                                                class="form-check-input"
                                                type="checkbox"
                                                checked
                                                disabled
                                            />
                                            <label class="form-check-label" style="width: 100%;">
                                                Title
                                            </label>
                                        </div>
                                    </div>
                                </li>
                                <li>
                                    <div class="dropdown-item">
                                        <div class="form-check" onclick="event.stopPropagation();">
                                            <input
                                                class="form-check-input"
                                                id="checkDateCreated"
                                                type="checkbox"
                                                checked=move || show_date_created.get()
                                                on:change=move |_| {
                                                    set_show_date_created.update(|v| *v = !*v)
                                                }
                                            />

                                            <label
                                                class="form-check-label"
                                                for="checkDateCreated"
                                                style="width: 100%;"
                                            >
                                                Date created
                                            </label>
                                        </div>
                                    </div>
                                </li>
                                <li>
                                    <div class="dropdown-item">
                                        <div class="form-check" onclick="event.stopPropagation();">
                                            <input
                                                class="form-check-input"
                                                id="checkAuthor"
                                                type="checkbox"
                                                checked=move || show_author.get()
                                                on:change=move |_| set_show_author.update(|v| *v = !*v)
                                            />
                                            <label
                                                class="form-check-label"
                                                for="checkAuthor"
                                                style="width: 100%;"
                                            >
                                                Author
                                            </label>
                                        </div>
                                    </div>
                                </li>
                                <li>
                                    <div class="dropdown-item">
                                        <div class="form-check" onclick="event.stopPropagation();">
                                            <input
                                                class="form-check-input"
                                                id="checkId"
                                                type="checkbox"
                                                checked=move || show_id.get()
                                                on:change=move |_| set_show_id.update(|v| *v = !*v)
                                            />
                                            <label
                                                class="form-check-label"
                                                for="checkId"
                                                style="width: 100%;"
                                            >
                                                ID
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

                    <table class="table table-striped">
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

                                {move || {
                                    if show_date_created.get() {
                                        view! {
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
                                        }
                                    } else {
                                        view! { <th></th> }
                                    }
                                }}

                                {move || {
                                    if show_author.get() {
                                        view! {
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
                                        }
                                    } else {
                                        view! { <th></th> }
                                    }
                                }}

                                {move || {
                                    if show_id.get() {
                                        view! {
                                            <th scope="col">
                                                <button
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
                                        }
                                    } else {
                                        view! { <th></th> }
                                    }
                                }}

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

                                                            {move || {
                                                                if show_date_created.get() {
                                                                    view! {
                                                                        <td>
                                                                            {post
                                                                                .date_created
                                                                                .format("%Y/%m/%d at %-I:%M %P")
                                                                                .to_string()}
                                                                        </td>
                                                                    }
                                                                } else {
                                                                    view! { <td></td> }
                                                                }
                                                            }}

                                                            {move || {
                                                                if show_author.get() {
                                                                    view! { <td>{post.author_id}</td> }
                                                                } else {
                                                                    view! { <td></td> }
                                                                }
                                                            }}

                                                            {move || {
                                                                if show_id.get() {
                                                                    view! { <td>{post.id}</td> }
                                                                } else {
                                                                    view! { <td></td> }
                                                                }
                                                            }}

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
