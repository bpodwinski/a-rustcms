use leptos::*;

use super::data_table_component::TableColumn;

#[component]
pub fn ColumnVisibilityDropdown<T: 'static + Clone>(
    columns: Signal<Vec<TableColumn<T>>>,
) -> impl IntoView {
    let total_columns = move || columns.get().len();
    let visible_columns = move || columns.get().iter().filter(|col| col.visible.get()).count();

    view! {
        <div class="dropdown me-2">
            <button
                class="btn btn-outline-primary dropdown-toggle"
                type="button"
                data-bs-toggle="dropdown"
                aria-expanded="false"
            >
                {move || format!("{}/{} Columns", visible_columns(), total_columns())}
            </button>
            <ul class="dropdown-menu">
                {move || {
                    let current_columns = columns.get_untracked();
                    current_columns
                        .into_iter()
                        .map(|column| {
                            let visible_signal = column.visible.clone();
                            view! {
                                <li>
                                    <div class="dropdown-item">
                                        <div class="form-check" onclick="event.stopPropagation();">
                                            <input
                                                class="form-check-input"
                                                type="checkbox"
                                                checked=move || visible_signal.get()
                                                on:change=move |_| {
                                                    visible_signal.update(|v| *v = !*v);
                                                }
                                            />

                                            <label class="form-check-label" style="width: 100%;">
                                                {column.title}
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
