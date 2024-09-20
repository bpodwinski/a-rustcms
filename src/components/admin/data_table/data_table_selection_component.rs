use ev::Event;
use leptos::*;
use std::collections::HashSet;

#[component]
pub fn TotalItems<T: 'static + Clone>(
    data: Signal<Vec<T>>,
    selected_datas: RwSignal<HashSet<u32>>,
) -> impl IntoView {
    view! {
        <caption aria-live="polite">
            {move || {
                let total_items = data.get().len();
                let selected_items = selected_datas.get().len();
                format!("{} selected / {} items", selected_items, total_items)
            }}

        </caption>
    }
}

#[component]
pub fn SelectAllCheckbox(
    data_ids: Signal<Vec<u32>>,
    selected_datas: RwSignal<HashSet<u32>>,
) -> impl IntoView {
    let is_all_selected = move || {
        let all_ids = data_ids
            .get_untracked()
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        !all_ids.is_empty() && selected_datas.get_untracked().len() == all_ids.len()
    };

    let toggle_select_all = move |ev: Event| {
        let checked = event_target_checked(&ev);
        if checked {
            let all_ids = data_ids
                .get_untracked()
                .iter()
                .cloned()
                .collect::<HashSet<_>>();
            selected_datas.set(all_ids);
        } else {
            selected_datas.set(HashSet::new());
        }
    };

    view! {
        <input
            class="form-check-input"
            type="checkbox"
            aria-label="Select all items"
            aria-checked=is_all_selected().to_string()
            on:change=toggle_select_all
            prop:checked=is_all_selected()
        />
    }
}

#[component]
pub fn Checkbox(data_id: u32, selected_datas: RwSignal<HashSet<u32>>) -> impl IntoView {
    let is_checked = move || selected_datas.get_untracked().contains(&data_id);

    let toggle_data_selection = move |_ev: Event| {
        selected_datas.update(|selected| {
            if selected.contains(&data_id) {
                selected.remove(&data_id);
            } else {
                selected.insert(data_id);
            }
        });
    };

    view! {
        <input
            class="form-check-input"
            type="checkbox"
            aria-label=format!("Select item {}", data_id)
            aria-checked=is_checked().to_string()
            on:change=toggle_data_selection
            prop:checked=is_checked()
        />
    }
}
