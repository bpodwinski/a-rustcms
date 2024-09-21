use leptos::*;
use leptos_router::{use_navigate, use_params_map, NavigateOptions};
use std::sync::Arc;

/// Component responsible for rendering pagination controls for a data table.
///
/// # Props
/// - `page`: The current page number being viewed (required).
/// - `page_count`: The total number of pages available (required).
/// - `on_page_change`: A callback function that triggers when the page changes.
///
/// This component displays a pagination bar that allows the user to navigate
/// through different pages. It dynamically calculates the visible page numbers
/// based on the current page and the total number of pages. The component also
/// ensures that the current page does not exceed the page bounds, automatically
/// redirecting if an invalid page is entered.
#[component]
pub fn DataTablePagination(
    /// Current page_num
    #[prop(into)]
    page: RwSignal<u32>,
    /// Total number of pages
    #[prop(into)]
    page_count: Signal<u32>,
    /// Callback when page changes
    on_page_change: Arc<dyn Fn(u32) + 'static>,
) -> impl IntoView {
    let navigate = use_navigate();
    let params = use_params_map();

    const MAX_VISIBLE_PAGES: u32 = 8;

    /// Navigates to the given page number and updates URL and state.
    ///
    /// # Arguments
    /// * `page_num` - The page number to navigate to
    /// * `page` - Signal storing the current page
    /// * `page_count` - The total number of pages
    /// * `on_page_change` - The callback executed when the page changes
    /// * `navigate` - Function to navigate to a new URL
    fn go_to_page(
        mut page_num: u32,
        page: RwSignal<u32>,
        page_count: u32,
        on_page_change: Arc<dyn Fn(u32) + 'static>,
        navigate: impl Fn(&str, NavigateOptions) + Clone + 'static,
    ) {
        // The current page must not exceed the page total
        page_num = page_num.clamp(1, page_count);

        // Signal update and navigation
        page.set(page_num);
        on_page_change(page_num);

        // Navigate to the new page URL
        let url = format!("rs-admin/posts/page/{}", page_num);
        let options = NavigateOptions {
            replace: true,
            ..NavigateOptions::default()
        };

        navigate(&url, options);
    }

    // Check and redirect if the page number is out of bounds
    let check_and_redirect_invalid_page = {
        let page = page.clone();
        let page_count = page_count.clone();
        let navigate = navigate.clone();
        let on_page_change = on_page_change.clone();

        move || {
            let total_pages_val = page_count.get();
            let current_page_val = page.get();

            if current_page_val > total_pages_val || current_page_val < 1 {
                go_to_page(
                    current_page_val.clamp(1, total_pages_val),
                    page,
                    total_pages_val,
                    on_page_change.clone(),
                    navigate.clone(),
                );
            }
        }
    };

    // Track changes in parameters and redirect if necessary
    params.track();
    check_and_redirect_invalid_page();

    /// Determines the range of pages visible in the pagination bar.
    ///
    /// # Arguments
    /// * `current` - The current page number
    /// * `total` - The total number of pages
    /// * `max_visible` - The maximum number of pages visible at once
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

    /// Creates a pagination item (individual page link or button).
    ///
    /// # Arguments
    /// * `label` - The text to display in the item
    /// * `page_num` - The page number associated with the item
    /// * `page_from_url` - The page number from the URL (for active status)
    /// * `page` - Signal storing the current page
    /// * `page_count` - The total number of pages
    /// * `on_page_change` - The callback executed when the page changes
    /// * `navigate` - Function to navigate to a new URL
    /// * `is_disabled` - Whether the pagination item should be disabled
    fn pagination_item(
        label: String,
        page_num: u32,
        page_from_url: u32,
        page: RwSignal<u32>,
        page_count: u32,
        on_page_change: Arc<dyn Fn(u32) + 'static>,
        navigate: impl Fn(&str, NavigateOptions) + Clone + 'static,
        is_disabled: bool,
    ) -> impl IntoView {
        let class = if is_disabled { "page-item disabled" } else { "page-item" };
        let active_class = if page_num == page_from_url { " active" } else { "" };
        let class_with_active = format!("{}{}", class, active_class);

        view! {
            <li class=class_with_active>
                <a
                    class="page-link"
                    href="#"
                    on:click=move |_| {
                        if !is_disabled {
                            go_to_page(
                                page_num,
                                page,
                                page_count,
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

    // Get the total number of pages and the current page number
    let total_pages_val = page_count.get();
    let current_page_val = page.get();
    let (start_page, end_page) = visible_page_range(current_page_val, total_pages_val, MAX_VISIBLE_PAGES);

    view! {
        <nav aria-label="Page navigation">
            <ul class="pagination justify-content-center">
                {pagination_item(
                    "Previous".to_string(),
                    current_page_val - 1,
                    current_page_val,
                    page,
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
                                current_page_val,
                                page,
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
                    .map(|page_num| {
                        let page_class = if page_num == current_page_val {
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
                                        page_num,
                                        page,
                                        total_pages_val,
                                        on_page_change_clone.clone(),
                                        navigate_clone.clone(),
                                    )
                                >

                                    {page_num.to_string()}
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
                                current_page_val,
                                page,
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
                    current_page_val,
                    page,
                    total_pages_val,
                    on_page_change,
                    navigate,
                    current_page_val == total_pages_val,
                )}

            </ul>
        </nav>
    }
}
