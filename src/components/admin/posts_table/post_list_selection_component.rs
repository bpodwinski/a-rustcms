use std::collections::HashSet;

use ev::Event;
use leptos::*;

use crate::models::admin::posts_model::PostStruct;

#[component]
pub fn TotalItems(
    posts: Signal<Vec<PostStruct>>,
    selected_posts: RwSignal<HashSet<u32>>,
) -> impl IntoView {
    view! {
        <caption>
            {move || {
                let total_items = posts.get().len();
                let selected_items = selected_posts.get().len();
                format!("{} selected / {} items", selected_items, total_items)
            }}

        </caption>
    }
}

#[component]
pub fn SelectAllCheckbox(
    posts: Signal<Vec<PostStruct>>,
    selected_posts: RwSignal<HashSet<u32>>,
) -> impl IntoView {
    let is_all_selected = move || {
        let all_ids = posts
            .get_untracked()
            .iter()
            .map(|post| post.id)
            .collect::<HashSet<_>>();
        !all_ids.is_empty() && selected_posts.get_untracked().len() == all_ids.len()
    };

    let toggle_select_all = move |ev: Event| {
        let checked = event_target_checked(&ev);
        if checked {
            let all_ids = posts
                .get_untracked()
                .iter()
                .map(|post| post.id)
                .collect::<HashSet<_>>();
            selected_posts.set(all_ids);
        } else {
            selected_posts.set(HashSet::new());
        }
    };

    view! {
        <input
            class="form-check-input"
            type="checkbox"
            aria-label="Select all posts"
            on:change=toggle_select_all
            prop:checked=is_all_selected()
        />
    }
}

#[component]
pub fn PostCheckbox(post_id: u32, selected_posts: RwSignal<HashSet<u32>>) -> impl IntoView {
    let is_checked = selected_posts.get_untracked().contains(&post_id);

    let toggle_post_selection = move |_ev: Event| {
        selected_posts.update(|selected| {
            if selected.contains(&post_id) {
                selected.remove(&post_id);
            } else {
                selected.insert(post_id);
            }
        });
    };

    view! {
        <input
            class="form-check-input"
            type="checkbox"
            on:change=toggle_post_selection
            prop:checked=is_checked
        />
    }
}
