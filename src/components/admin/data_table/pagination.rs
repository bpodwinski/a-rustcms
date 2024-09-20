use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
use std::sync::Arc;

#[component]
pub fn DataTablePagination(
    current_page: RwSignal<u32>,
    total_pages: Signal<u32>,
    on_page_change: Arc<dyn Fn(u32) + 'static>,
) -> impl IntoView {
    let navigate = use_navigate();

    const MAX_VISIBLE_PAGES: u32 = 8;

    fn go_to_page(
        page: u32,
        current_page: RwSignal<u32>,
        total_pages: u32,
        on_page_change: Arc<dyn Fn(u32) + 'static>,
        navigate: impl Fn(&str, NavigateOptions) + Clone + 'static,
    ) {
        if page > 0 && page <= total_pages {
            current_page.set(page);
            on_page_change(page);
            let url = format!("rs-admin/posts/page/{}", page);
            let options = NavigateOptions::default();
            navigate(&url, options);
        }
    }

    fn visible_page_range(current: u32, total: u32, max_visible: u32) -> (u32, u32) {
        let half_visible = max_visible / 2;
        let start = if current <= half_visible {
            1
        } else if current + half_visible >= total {
            total.saturating_sub(max_visible - 1)
        } else {
            current.saturating_sub(half_visible)
        };
        let end = u32::min(start + max_visible - 1, total);
        (start, end)
    }

    fn pagination_item(
        label: String,
        page: u32,
        current_page: RwSignal<u32>,
        total_pages: u32,
        on_page_change: Arc<dyn Fn(u32) + 'static>,
        navigate: impl Fn(&str, NavigateOptions) + Clone + 'static,
        is_disabled: bool,
    ) -> impl IntoView {
        let class = if is_disabled {
            "page-item disabled"
        } else {
            "page-item"
        };
        view! {
            <li class=class>
                <a
                    class="page-link"
                    href="#"
                    on:click=move |_| {
                        if !is_disabled {
                            go_to_page(
                                page,
                                current_page,
                                total_pages,
                                on_page_change.clone(),
                                navigate.clone(),
                            );
                        }
                    }
                >

                    {label}
                </a>
            </li>
        }
    }

    let total_pages_val = total_pages.get_untracked();
    let current_page_val = current_page.get_untracked();
    let (start_page, end_page) =
        visible_page_range(current_page_val, total_pages_val, MAX_VISIBLE_PAGES);

    view! {
        <nav aria-label="Page navigation">
            <ul class="pagination justify-content-center">
                {pagination_item(
                    "Previous".to_string(),
                    current_page_val - 1,
                    current_page,
                    total_pages_val,
                    on_page_change.clone(),
                    navigate.clone(),
                    current_page_val == 1,
                )}
                {if start_page > 1 {
                    view! {
                        <>
                            {pagination_item(
                                "1".to_string(),
                                1,
                                current_page,
                                total_pages_val,
                                on_page_change.clone(),
                                navigate.clone(),
                                false,
                            )} <li class="page-item disabled">
                                <span class="page-link">{"..."}</span>
                            </li>
                        </>
                    }
                } else {
                    view! {
                        <>
                            <li></li>
                        </>
                    }
                }}
                {(start_page..=end_page)
                    .map(|page| {
                        let page_class = if page == current_page_val {
                            "page-item active"
                        } else {
                            "page-item"
                        };
                        let on_page_change_clone = on_page_change.clone();
                        let navigate_clone = navigate.clone();
                        view! {
                            <li class=page_class>
                                <a
                                    class="page-link"
                                    href="#"
                                    on:click=move |_| go_to_page(
                                        page,
                                        current_page,
                                        total_pages_val,
                                        on_page_change_clone.clone(),
                                        navigate_clone.clone(),
                                    )
                                >

                                    {page.to_string()}
                                </a>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_view()}
                {if end_page < total_pages_val {
                    view! {
                        <>
                            <li class="page-item disabled">
                                <span class="page-link">{"..."}</span>
                            </li>
                            {pagination_item(
                                total_pages_val.to_string(),
                                total_pages_val,
                                current_page,
                                total_pages_val,
                                on_page_change.clone(),
                                navigate.clone(),
                                false,
                            )}
                        </>
                    }
                } else {
                    view! {
                        <>
                            <li></li>
                        </>
                    }
                }}
                {pagination_item(
                    "Next".to_string(),
                    current_page_val + 1,
                    current_page,
                    total_pages_val,
                    on_page_change,
                    navigate,
                    current_page_val == total_pages_val,
                )}

            </ul>
        </nav>
    }
}
