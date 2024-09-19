use leptos::*;
use std::{collections::HashSet, sync::Arc};

use crate::{
    components::admin::posts_table::{
        post_list_column_visibility_component::ColumnVisibilityDropdown,
        post_list_selection_component::*, post_list_sorts_component::*,
        post_list_table_header_component::PostTableHeader,
    },
    models::admin::posts_model::PostStruct,
};

#[derive(Clone)]
pub struct TableColumn<T> {
    pub title: &'static str,
    pub value_fn: Arc<dyn Fn(&T) -> String + Send + Sync>,
    pub visible: RwSignal<bool>,
    pub sort_column: Option<SortColumn>,
}

#[component]
pub fn PostListV2(
    posts: Signal<Vec<PostStruct>>,
    selected_posts: RwSignal<HashSet<u32>>,
) -> impl IntoView {
    let sort_column: RwSignal<SortColumn> = create_rw_signal(SortColumn::ID);
    let sort_order = create_rw_signal(SortOrder::Descending);

    let (columns, _) = create_signal(vec![
        TableColumn {
            title: "Title",
            value_fn: Arc::new(|post: &PostStruct| post.title.clone()),
            visible: create_rw_signal(true),
            sort_column: Some(SortColumn::Title),
        },
        TableColumn {
            title: "Date Created",
            value_fn: Arc::new(|post: &PostStruct| {
                post.date_created.format("%Y/%m/%d").to_string()
            }),
            visible: create_rw_signal(true),
            sort_column: Some(SortColumn::DateCreated),
        },
        TableColumn {
            title: "Author",
            value_fn: Arc::new(|post: &PostStruct| post.author_id.to_string()),
            visible: create_rw_signal(true),
            sort_column: Some(SortColumn::Author),
        },
        TableColumn {
            title: "ID",
            value_fn: Arc::new(|post: &PostStruct| post.id.to_string()),
            visible: create_rw_signal(true),
            sort_column: Some(SortColumn::ID),
        },
    ]);

    view! {
        <div class="card mb-3">
            <div class="card-body">
                <div class="content-list">

                    <div class="d-flex justify-content-end align-items-center w-100 my-2">

                        <SortSelect sort_column=sort_column sort_order=sort_order/>

                        <ColumnVisibilityDropdown columns=columns.into()/>

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

                        <PostTableHeader
                            sort_column=sort_column
                            sort_order=sort_order
                            columns=columns.into()
                            posts=posts
                            selected_posts=selected_posts
                        />

                        <tbody>
                            {move || {
                                let mut post_list = posts.get();
                                sort_posts(&mut post_list, sort_column.get(), sort_order.get());
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
                                                                <PostCheckbox
                                                                    post_id=post.id
                                                                    selected_posts=selected_posts
                                                                />
                                                            </td>

                                                            {columns
                                                                .get()
                                                                .iter()
                                                                .filter(|c| c.visible.get())
                                                                .map(|column| {
                                                                    view! { <td>{(column.value_fn)(&post)}</td> }
                                                                })
                                                                .collect::<Vec<_>>()}

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

                        <TotalItems posts=posts selected_posts=selected_posts/>

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
