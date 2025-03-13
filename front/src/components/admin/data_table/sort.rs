use leptos::*;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

pub fn toggle_sort(
    sort_column: RwSignal<Option<usize>>,
    sort_order: RwSignal<SortOrder>,
    index: usize,
    on_sort_change: impl Fn(Option<usize>, SortOrder) + Clone + 'static,
) {
    if let Some(current_index) = sort_column.get() {
        if current_index == index {
            if sort_order.get() == SortOrder::Ascending {
                sort_order.set(SortOrder::Descending);
            } else {
                sort_order.set(SortOrder::Ascending);
            }
        } else {
            sort_column.set(Some(index));
            sort_order.set(SortOrder::Ascending);
        }
    } else {
        sort_column.set(Some(index));
        sort_order.set(SortOrder::Ascending);
    }

    on_sort_change(Some(index), sort_order.get());
}

/* #[component]
pub fn DataTableSortSelect<T: 'static + Clone>(
    columns: Signal<Vec<TableColumn<T>>>,
    sort_column: RwSignal<Option<usize>>,
    sort_order: RwSignal<SortOrder>,
    on_sort_change: impl Fn(String, String) + Clone + 'static,
) -> impl IntoView {
    view! {
        <select
            class="form-select me-2"
            aria-label="Sort options"
            style="width: fit-content"
            on:change=move |ev| {
                if let Ok(selected_index) = event_target_value(&ev).parse::<usize>() {
                    let column_title = columns.get()[selected_index].title.to_string();
                    if sort_column.get() != Some(selected_index) {
                        sort_order.set(SortOrder::Ascending);
                    }
                    toggle_sort(
                        sort_column,
                        sort_order,
                        selected_index,
                        column_title.clone(),
                        on_sort_change.clone(),
                    );
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
} */
