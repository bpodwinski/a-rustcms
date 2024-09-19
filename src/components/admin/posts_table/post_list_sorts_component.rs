use leptos::*;

use super::post_list_v2_component::TableColumn;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

pub fn toggle_sort(
    sort_column: RwSignal<Option<usize>>,
    sort_order: RwSignal<SortOrder>,
    column_index: usize,
) {
    if sort_column.get() == Some(column_index) {
        sort_order.update(|order| {
            *order = match *order {
                SortOrder::Ascending => SortOrder::Descending,
                SortOrder::Descending => SortOrder::Ascending,
            };
        });
    } else {
        sort_column.set(Some(column_index));
        sort_order.set(SortOrder::Ascending);
    }
}

pub fn sort_posts<T>(
    posts: &mut Vec<T>,
    columns: &Vec<TableColumn<T>>,
    sort_column: Option<usize>,
    sort_order: SortOrder,
) {
    if let Some(col_index) = sort_column {
        if let Some(sort_fn) = &columns[col_index].sort_fn {
            match sort_order {
                SortOrder::Ascending => posts.sort_by(|a, b| sort_fn(a, b)),
                SortOrder::Descending => posts.sort_by(|a, b| sort_fn(b, a)),
            }
        }
    }
}

#[component]
pub fn SortSelect<T: 'static + Clone>(
    columns: Signal<Vec<TableColumn<T>>>,
    sort_column: RwSignal<Option<usize>>,
    sort_order: RwSignal<SortOrder>,
) -> impl IntoView {
    view! {
        <select
            class="form-select me-2"
            aria-label="Sort Table By"
            style="width: fit-content;"
            on:change=move |ev| {
                if let Ok(selected_index) = event_target_value(&ev).parse::<usize>() {
                    toggle_sort(sort_column, sort_order, selected_index);
                }
            }
        >

            {move || {
                columns
                    .get()
                    .iter()
                    .enumerate()
                    .filter_map(|(index, col)| {
                        col.sort_fn
                            .as_ref()
                            .map(|_| {
                                view! {
                                    <option
                                        value=index
                                        selected=move || sort_column.get() == Some(index)
                                    >
                                        {format!(
                                            "{} ({})",
                                            col.title,
                                            if sort_order.get() == SortOrder::Ascending {
                                                "ascending"
                                            } else {
                                                "descending"
                                            },
                                        )}

                                    </option>
                                }
                            })
                    })
                    .collect::<Vec<_>>()
            }}

        </select>
    }
}
