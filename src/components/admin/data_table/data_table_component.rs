use leptos::*;
use std::{collections::HashSet, sync::Arc};

use crate::{
    components::admin::data_table::{
        data_table_column_visibility_component::ColumnVisibilityDropdown,
        data_table_header_component::DataTableHeader,
        data_table_selection_component::{Checkbox, TotalItems},
        data_table_sorts_component::*,
    },
    models::admin::posts_model::Id,
};

#[derive(Clone)]
pub struct TableColumn<T> {
    pub title: &'static str,
    pub value_fn: Arc<dyn Fn(&T) -> View + Send + Sync>,
    pub sort_fn: Option<Arc<dyn Fn(&T, &T) -> std::cmp::Ordering + Send + Sync>>,
    pub visible: RwSignal<bool>,
}

#[component]
pub fn DataTable<T: Id + 'static + Clone>(
    data: Signal<Vec<T>>,
    columns: Signal<Vec<TableColumn<T>>>,
    selected_datas: RwSignal<HashSet<u32>>,
    total_items: Signal<u32>,
    current_page: RwSignal<u32>,
    items_per_page: RwSignal<u32>,
    on_page_change: Arc<dyn Fn(u32)>,
    on_items_per_page_change: Arc<dyn Fn(u32)>,
) -> impl IntoView {
    let sort_column: RwSignal<Option<usize>> = create_rw_signal(None);
    let sort_order = create_rw_signal(SortOrder::Descending);

    // Pagination
    let total_pages = create_memo(move |_| {
        let total_items = total_items.get();
        let per_page = items_per_page.get();
        if per_page > 0 {
            (total_items + per_page - 1) / per_page
        } else {
            1
        }
    });

    fn go_to_page(
        page: u32,
        current_page: &RwSignal<u32>,
        total_pages: u32,
        on_page_change: Arc<dyn Fn(u32)>,
    ) {
        if page > 0 && page <= total_pages {
            current_page.set(page);
            on_page_change(page);
        }
    }

    view! {
        <div class="card mb-3" aria-live="polite">
            <div class="card-body">
                <div class="content-list">

                    <div class="d-flex justify-content-end align-items-center w-100 my-2">

                        <SortSelect
                            columns=columns.into()
                            sort_column=sort_column
                            sort_order=sort_order
                        />

                        <ColumnVisibilityDropdown columns=columns.into()/>

                        // Select for items per page
                        <select
                            class="form-select"
                            aria-label="Items per page"
                            style="width: fit-content"
                            on:change=move |ev| {
                                let new_items_per_page = event_target_value(&ev)
                                    .parse::<u32>()
                                    .unwrap_or(10);
                                on_items_per_page_change(new_items_per_page);
                            }
                        >

                            <option value="5" selected=move || items_per_page.get() == 5>
                                {"5"}
                            </option>
                            <option value="10" selected=move || items_per_page.get() == 10>
                                {"10"}
                            </option>
                            <option value="20" selected=move || items_per_page.get() == 20>
                                {"20"}
                            </option>
                            <option value="50" selected=move || items_per_page.get() == 50>
                                {"50"}
                            </option>
                            <option value="100" selected=move || items_per_page.get() == 100>
                                {"100"}
                            </option>
                        </select>

                    </div>

                    <table class="table table-striped" aria-live="polite">

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
                                                .iter()
                                                .map(|data| {
                                                    let data_id = data.id();
                                                    let is_checked = selected_datas.get().contains(&data_id);
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
                                                                        if set.contains(&data_id) {
                                                                            set.remove(&data_id);
                                                                        } else {
                                                                            set.insert(data_id);
                                                                        }
                                                                    });
                                                            }

                                                            aria-selected=is_checked.to_string()
                                                        >

                                                            <td>
                                                                <Checkbox data_id=data_id selected_datas=selected_datas/>
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

                    // Pagination component
                    <nav aria-label="Page navigation example">
                        <ul class="pagination">
                            <li class=move || {
                                if current_page.get() == 1 {
                                    "page-item disabled"
                                } else {
                                    "page-item"
                                }
                            }>

                                {
                                    let on_page_change_clone = on_page_change.clone();
                                    view! {
                                        <a
                                            class="page-link"
                                            href="#"
                                            on:click=move |_| go_to_page(
                                                current_page.get() - 1,
                                                &current_page,
                                                total_pages.get(),
                                                on_page_change_clone.clone(),
                                            )
                                        >

                                            "Previous"
                                        </a>
                                    }
                                }

                            </li>

                            {
                                let total_pages_val = total_pages.get_untracked();
                                let current_page_val = current_page.get_untracked();
                                (1..=total_pages_val)
                                    .map(|page| {
                                        let page_class = if page == current_page_val {
                                            "page-item active"
                                        } else {
                                            "page-item"
                                        };
                                        let on_page_change_clone = on_page_change.clone();
                                        view! {
                                            <li class=page_class>
                                                <a
                                                    class="page-link"
                                                    href="#"
                                                    on:click=move |_| go_to_page(
                                                        page,
                                                        &current_page,
                                                        total_pages_val,
                                                        on_page_change_clone.clone(),
                                                    )
                                                >

                                                    {page.to_string()}
                                                </a>
                                            </li>
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                    .into_view()
                            }

                            <li class=move || {
                                if current_page.get() == total_pages.get() {
                                    "page-item disabled"
                                } else {
                                    "page-item"
                                }
                            }>

                                {
                                    let on_page_change_clone = on_page_change.clone();
                                    view! {
                                        <a
                                            class="page-link"
                                            href="#"
                                            on:click=move |_| go_to_page(
                                                current_page.get() + 1,
                                                &current_page,
                                                total_pages.get(),
                                                on_page_change_clone.clone(),
                                            )
                                        >

                                            "Next"
                                        </a>
                                    }
                                }

                            </li>
                        </ul>
                    </nav>

                </div>
            </div>
        </div>
    }
}
