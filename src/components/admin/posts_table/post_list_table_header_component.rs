use std::collections::HashSet;

use leptos::*;

use crate::{
    components::admin::posts_table::{
        post_list_selection_component::SelectAllCheckbox, post_list_sorts_component::*,
    },
    models::admin::posts_model::PostStruct,
};

use super::post_list_v2_component::TableColumn;

#[component]
pub fn PostTableHeader<T: 'static + Clone>(
    sort_column: RwSignal<Option<usize>>,
    sort_order: RwSignal<SortOrder>,
    columns: Signal<Vec<TableColumn<T>>>,
    posts: Signal<Vec<PostStruct>>,
    selected_posts: RwSignal<HashSet<u32>>,
) -> impl IntoView {
    view! {
        <thead>
            <tr>

                <th scope="col">
                    <SelectAllCheckbox posts=posts selected_posts=selected_posts/>
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
