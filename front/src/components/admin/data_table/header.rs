use leptos::*;
use std::collections::HashSet;

use super::{data_table_component::TableColumn, sort::SortOrder};
use crate::{
    components::admin::data_table::{selection::DataTableSelectAllCheckbox, sort::toggle_sort},
    models::admin::posts_model::Id,
};

#[component]
pub fn DataTableHeader<T: Id + 'static + Clone>(
    sort_column: RwSignal<Option<usize>>,
    sort_order: RwSignal<SortOrder>,
    columns: Signal<Vec<TableColumn<T>>>,
    data: Signal<Vec<T>>,
    selected_datas: RwSignal<HashSet<u32>>,
    on_sort_change: impl Fn(Option<usize>, SortOrder) + Clone + 'static,
) -> impl IntoView {
    view! {
        <thead>
            <tr>

                <th scope="col">
                    <DataTableSelectAllCheckbox
                        data_ids=Signal::derive(move || {
                            data.get().iter().map(|item| item.id()).collect::<Vec<u32>>()
                        })

                        selected_datas=selected_datas
                    />
                </th>

                {move || {
                    let columns = columns.get().to_owned();
                    columns
                        .iter()
                        .enumerate()
                        .map(|(index, column)| {
                            if column.visible.get() {
                                let on_sort_change = on_sort_change.clone();
                                Some(
                                    view! {
                                        <th scope="col">

                                            <button
                                                class="btn btn-dark"
                                                on:click=move |_| toggle_sort(
                                                    sort_column,
                                                    sort_order,
                                                    index,
                                                    on_sort_change.clone(),
                                                )
                                            >

                                                {column.title}

                                                {move || {
                                                    if sort_column.get() == Some(index) {
                                                        if sort_order.get() == SortOrder::Ascending {
                                                            view! { <i class="bi bi-sort-down-alt ms-1"></i> }
                                                        } else {
                                                            view! { <i class="bi bi-sort-up ms-1"></i> }
                                                        }
                                                    } else {
                                                        view! { <i class="bi bi-sort-up text-secondary ms-1"></i> }
                                                    }
                                                }}

                                            </button>

                                        </th>
                                    },
                                )
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                }}

            </tr>
        </thead>
    }
}
