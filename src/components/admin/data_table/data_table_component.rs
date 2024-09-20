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
    total_items: Signal<usize>,
    on_page_change: Arc<dyn Fn(u32)>,
    on_items_per_page_change: Arc<dyn Fn(u32)>,
) -> impl IntoView {
    let sort_column: RwSignal<Option<usize>> = create_rw_signal(None);
    let sort_order = create_rw_signal(SortOrder::Descending);
    let current_page = create_rw_signal(1);
    let items_per_page = create_rw_signal(10);
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
                </div>
            </div>
        </div>
    }
}
