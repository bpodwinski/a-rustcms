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
    sort_column: RwSignal<SortColumn>,
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
                        .into_iter()
                        .map(|column| {
                            if column.visible.get() {
                                view! {
                                    <th scope="col">
                                        {if let Some(sort_col) = column.sort_column {
                                            view! {
                                                <button
                                                    class="btn btn-dark"
                                                    on:click=move |_| toggle_sort(
                                                        sort_column,
                                                        sort_order,
                                                        sort_col,
                                                    )
                                                >

                                                    {column.title}
                                                    {move || {
                                                        if sort_column.get() == sort_col {
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
                                            }
                                        } else {
                                            view! {
                                                <button class="btn btn-dark">{column.title}</button>
                                            }
                                        }}

                                    </th>
                                }
                            } else {
                                view! { <th></th> }
                            }
                        })
                        .collect::<Vec<_>>()
                }}

            </tr>
        </thead>
    }
}
