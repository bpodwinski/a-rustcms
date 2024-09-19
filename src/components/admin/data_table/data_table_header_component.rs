use std::collections::HashSet;

use leptos::*;

use crate::{
    components::admin::data_table::{
        data_table_selection_component::SelectAllCheckbox, data_table_sorts_component::toggle_sort,
    },
    models::admin::posts_model::Id,
};

use super::{data_table_component::TableColumn, data_table_sorts_component::SortOrder};

#[component]
pub fn DataTableHeader<T: Id + 'static + Clone>(
    sort_column: RwSignal<Option<usize>>,
    sort_order: RwSignal<SortOrder>,
    columns: Signal<Vec<TableColumn<T>>>,
    data: Signal<Vec<T>>,
    selected_datas: RwSignal<HashSet<u32>>,
) -> impl IntoView {
    view! {
        <thead>
            <tr>

                <th scope="col">
                    <SelectAllCheckbox
                        data_ids=Signal::derive(move || {
                            data.get().iter().map(|item| item.id()).collect::<Vec<u32>>()
                        })

                        selected_datas=selected_datas
                    />
                </th>

                {move || {
                    columns
                        .get()
                        .iter()
                        .enumerate()
                        .map(|(index, column)| {
                            if column.visible.get() {
                                Some(
                                    view! {
                                        <th scope="col">
                                            {if column.sort_fn.is_some() {
                                                view! {
                                                    <button
                                                        class="btn btn-dark"
                                                        on:click=move |_| toggle_sort(
                                                            sort_column,
                                                            sort_order,
                                                            index,
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
                                                }
                                            } else {
                                                view! { <button>{column.title}</button> }
                                            }}

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
