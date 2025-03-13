use leptos::*;

use super::data_table_component::TableColumn;

#[component]
pub fn DataTableVisibilityDropdown<T: 'static + Clone>(
    columns: Signal<Vec<TableColumn<T>>>,
) -> impl IntoView {
    let total_columns = move || columns.get().len();
    let visible_columns = move || columns.get().iter().filter(|col| col.visible.get()).count();

    view! {
        <div class="dropdown me-2">
            <button
                class="btn btn-primary dropdown-toggle"
                type="button"
                data-bs-toggle="dropdown"
                aria-expanded="false"
                aria-label="Toggle column visibility"
            >
                {move || format!("{}/{} Columns", visible_columns(), total_columns())}
            </button>
            <ul class="dropdown-menu" aria-label="Column visibility options">
                {move || {
                    let current_columns = columns.get_untracked();
                    current_columns
                        .into_iter()
                        .map(|column| {
                            let visible_signal = column.visible.clone();
                            let column_title = column.title;
                            view! {
                                <li>
                                    <div class="dropdown-item">
                                        <div class="form-check" onclick="event.stopPropagation();">
                                            <input
                                                class="form-check-input"
                                                type="checkbox"
                                                aria-label=format!("Toggle visibility of {}", column_title)
                                                checked=move || visible_signal.get()
                                                aria-checked=move || visible_signal.get().to_string()
                                                on:change=move |_| {
                                                    visible_signal.update(|v| *v = !*v);
                                                }
                                            />

                                            <label class="form-check-label" style="width: 100%">
                                                {column_title}
                                            </label>
                                        </div>
                                    </div>
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()
                }}

            </ul>
        </div>
    }
}
