use leptos::*;
use std::{collections::HashSet, sync::Arc};

use crate::{
    components::admin::data_table::{
        data_table_column_visibility_component::ColumnVisibilityDropdown,
        data_table_header_component::DataTableHeader,
        data_table_pagination_component::Pagination,
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

    let total_pages = create_memo(move |_| {
        let total_items = total_items.get();
        let per_page = items_per_page.get();
        if per_page > 0 {
            (total_items + per_page - 1) / per_page
        } else {
            1
        }
    });

    view! {
        <div class="card mb-3" aria-live="polite">
            <div class="card-body">
                <div class="content-list">

                    <div class="d-flex justify-content-end align-items-center w-100 my-2">

                        <div class="input-group me-2" style="width: fit-content">
                            <input
                                type="text"
                                class="form-control"
                                placeholder="Search"
                                aria-label="Search"
                                aria-describedby="Search in datas"
                                style="width: 300px"
                            />
                            <button class="btn btn-primary" type="button" id="button-addon2">
                                <i class="bi bi-search"></i>
                            </button>
                        </div>

                        <a class="btn btn-primary me-2" href="#" role="button">
                            Filters
                        </a>

                        <SortSelect
                            columns=columns.into()
                            sort_column=sort_column
                            sort_order=sort_order
                        />

                        <select
                            class="form-select me-2"
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

                        <ColumnVisibilityDropdown columns=columns.into()/>

                    </div>

                    // Table rendering
                    <table class="table" aria-live="polite">

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

                    <Pagination
                        current_page=current_page
                        total_pages=total_pages.into()
                        on_page_change=on_page_change.clone()
                    />

                </div>
            </div>
        </div>
    }
}
