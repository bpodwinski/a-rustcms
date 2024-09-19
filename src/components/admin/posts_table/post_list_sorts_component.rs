use ev::Event;
use leptos::*;

use crate::models::admin::posts_model::PostStruct;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortColumn {
    Title,
    DateCreated,
    Author,
    ID,
}

pub fn toggle_sort(
    sort_column: RwSignal<SortColumn>,
    sort_order: RwSignal<SortOrder>,
    column: SortColumn,
) {
    if sort_column.get() == column {
        sort_order.update(|order| {
            if *order == SortOrder::Ascending {
                *order = SortOrder::Descending;
            } else {
                *order = SortOrder::Ascending;
            }
        });
    } else {
        sort_column.set(column);
        sort_order.set(SortOrder::Ascending);
    }
}

pub fn sort_posts(posts: &mut Vec<PostStruct>, column: SortColumn, order: SortOrder) {
    match column {
        SortColumn::Title => {
            if order == SortOrder::Ascending {
                posts.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
            } else {
                posts.sort_by(|a, b| b.title.to_lowercase().cmp(&a.title.to_lowercase()));
            }
        }
        SortColumn::DateCreated => {
            if order == SortOrder::Ascending {
                posts.sort_by(|a, b| a.date_created.cmp(&b.date_created));
            } else {
                posts.sort_by(|a, b| b.date_created.cmp(&a.date_created));
            }
        }
        SortColumn::Author => {
            if order == SortOrder::Ascending {
                posts.sort_by(|a, b| a.author_id.cmp(&b.author_id));
            } else {
                posts.sort_by(|a, b| b.author_id.cmp(&a.author_id));
            }
        }
        SortColumn::ID => {
            if order == SortOrder::Ascending {
                posts.sort_by(|a, b| a.id.cmp(&b.id));
            } else {
                posts.sort_by(|a, b| b.id.cmp(&a.id));
            }
        }
    }
}

pub fn current_sort_value(
    sort_column: RwSignal<SortColumn>,
    sort_order: RwSignal<SortOrder>,
) -> String {
    match sort_column.get() {
        SortColumn::Title => match sort_order.get() {
            SortOrder::Ascending => "TitleAscending".to_string(),
            SortOrder::Descending => "TitleDescending".to_string(),
        },
        SortColumn::ID => match sort_order.get() {
            SortOrder::Ascending => "IdAscending".to_string(),
            SortOrder::Descending => "IdDescending".to_string(),
        },
        SortColumn::DateCreated => match sort_order.get() {
            SortOrder::Ascending => "DateCreatedAscending".to_string(),
            SortOrder::Descending => "DateCreatedDescending".to_string(),
        },
        SortColumn::Author => match sort_order.get() {
            SortOrder::Ascending => "AuthorAscending".to_string(),
            SortOrder::Descending => "AuthorDescending".to_string(),
        },
    }
}

pub fn handle_sort_change(
    sort_column: RwSignal<SortColumn>,
    sort_order: RwSignal<SortOrder>,
    ev: Event,
) {
    let value = event_target_value(&ev);
    let (column, order) = match value.as_str() {
        "TitleAscending" => (SortColumn::Title, SortOrder::Ascending),
        "TitleDescending" => (SortColumn::Title, SortOrder::Descending),
        "IdAscending" => (SortColumn::ID, SortOrder::Ascending),
        "IdDescending" => (SortColumn::ID, SortOrder::Descending),
        "DateCreatedAscending" => (SortColumn::DateCreated, SortOrder::Ascending),
        "DateCreatedDescending" => (SortColumn::DateCreated, SortOrder::Descending),
        "AuthorAscending" => (SortColumn::Author, SortOrder::Ascending),
        "AuthorDescending" => (SortColumn::Author, SortOrder::Descending),
        _ => (SortColumn::ID, SortOrder::Descending),
    };
    sort_column.set(column);
    sort_order.set(order);
}

#[component]
pub fn SortSelect(
    sort_column: RwSignal<SortColumn>,
    sort_order: RwSignal<SortOrder>,
) -> impl IntoView {
    view! {
        <select
            class="form-select me-2"
            aria-label="Sort Table By"
            style="width: fit-content;"
            on:change=move |ev| handle_sort_change(sort_column, sort_order, ev)
        >
            <option
                value="TitleAscending"
                selected=move || current_sort_value(sort_column, sort_order) == "TitleAscending"
            >
                Title ascending
            </option>
            <option
                value="TitleDescending"
                selected=move || current_sort_value(sort_column, sort_order) == "TitleDescending"
            >
                Title descending
            </option>
            <option
                value="DateCreatedAscending"
                selected=move || {
                    current_sort_value(sort_column, sort_order) == "DateCreatedAscending"
                }
            >
                Date created ascending
            </option>
            <option
                value="DateCreatedDescending"
                selected=move || {
                    current_sort_value(sort_column, sort_order) == "DateCreatedDescending"
                }
            >
                Date created descending
            </option>
            <option
                value="AuthorAscending"
                selected=move || current_sort_value(sort_column, sort_order) == "AuthorAscending"
            >
                Author ascending
            </option>
            <option
                value="AuthorDescending"
                selected=move || current_sort_value(sort_column, sort_order) == "AuthorDescending"
            >
                Author descending
            </option>
            <option
                value="IdAscending"
                selected=move || current_sort_value(sort_column, sort_order) == "IdAscending"
            >
                ID ascending
            </option>
            <option
                value="IdDescending"
                selected=move || current_sort_value(sort_column, sort_order) == "IdDescending"
            >
                ID descending
            </option>
        </select>
    }
}
