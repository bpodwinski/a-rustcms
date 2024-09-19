use leptos::*;
use std::{collections::HashSet, sync::Arc};

use crate::{
    components::admin::data_table::{
        data_table_column_visibility_component::ColumnVisibilityDropdown,
        data_table_header_component::DataTableHeader,
        data_table_selection_component::{Checkbox, TotalItems},
        data_table_sorts_component::*,
    },
    models::admin::posts_model::PostStruct,
};

#[derive(Clone)]
pub struct TableColumn<T> {
    pub title: &'static str,
    pub value_fn: Arc<dyn Fn(&T) -> View + Send + Sync>,
    pub sort_fn: Option<Arc<dyn Fn(&T, &T) -> std::cmp::Ordering + Send + Sync>>,
    pub visible: RwSignal<bool>,
}

#[component]
pub fn DataTable(
    data: Signal<Vec<PostStruct>>,
    selected_datas: RwSignal<HashSet<u32>>,
) -> impl IntoView {
    let sort_column: RwSignal<Option<usize>> = create_rw_signal(None);
    let sort_order = create_rw_signal(SortOrder::Descending);

    let (columns, _) = create_signal(vec![
        TableColumn {
            title: "Title",
            value_fn: Arc::new(|post: &PostStruct| {
                view! {
                    <>
                        <a href=format!("/rs-admin/posts/{}/edit", post.id)>
                            {&post.title}
                        </a>
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
                        {post.date_created.format("%Y/%m/%d").to_string()}<br/>
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
            value_fn: Arc::new(|post: &PostStruct| view! { <> {post.author_id} </> }.into()),
            sort_fn: Some(Arc::new(|a, b| a.author_id.cmp(&b.author_id))),
            visible: create_rw_signal(true),
        },
        TableColumn {
            title: "ID",
            value_fn: Arc::new(|post: &PostStruct| view! { <> {post.id} </> }.into()),
            sort_fn: Some(Arc::new(|a, b| a.id.cmp(&b.id))),
            visible: create_rw_signal(true),
        },
    ]);

    view! {
        <div class="card mb-3">
            <div class="card-body">
                <div class="content-list">

                    <div class="d-flex justify-content-end align-items-center w-100 my-2">

                        <SortSelect
                            columns=columns.into()
                            sort_column=sort_column
                            sort_order=sort_order
                        />

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

                        <DataTableHeader
                            sort_column=sort_column
                            sort_order=sort_order
                            columns=columns.into()
                            data=data
                            selected_datas=selected_datas
                        />

                        <tbody>
                            {move || {
                                let mut data_list = data.get();
                                sort_datas(
                                    &mut data_list,
                                    &columns.get(),
                                    sort_column.get(),
                                    sort_order.get(),
                                );
                                if !data_list.is_empty() {
                                    view! {
                                        <>
                                            {data_list
                                                .into_iter()
                                                .map(|data| {
                                                    let is_checked = selected_datas.get().contains(&data.id);
                                                    let row_class = if is_checked {
                                                        "table-active"
                                                    } else {
                                                        ""
                                                    };
                                                    view! {
                                                        <tr
                                                            class=row_class
                                                            on:click=move |_| {
                                                                selected_datas
                                                                    .update(|set| {
                                                                        if set.contains(&data.id) {
                                                                            set.remove(&data.id);
                                                                        } else {
                                                                            set.insert(data.id);
                                                                        }
                                                                    });
                                                            }
                                                        >

                                                            <td>
                                                                <Checkbox data_id=data.id selected_datas=selected_datas/>
                                                            </td>

                                                            {columns
                                                                .get()
                                                                .iter()
                                                                .filter(|c| c.visible.get())
                                                                .map(|column| {
                                                                    view! { <td>{(column.value_fn)(&data)}</td> }
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
                                                <td colspan="5">"No datas available"</td>
                                            </tr>
                                        </>
                                    }
                                }
                            }}

                        </tbody>

                        <TotalItems data=data selected_datas=selected_datas/>

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
