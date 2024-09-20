use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
use std::sync::Arc;

#[component]
pub fn Pagination(
    current_page: RwSignal<u32>,
    total_pages: Signal<u32>,
    on_page_change: Arc<dyn Fn(u32)>,
) -> impl IntoView {
    let navigate = use_navigate();

    fn go_to_page(
        page: u32,
        current_page: &RwSignal<u32>,
        total_pages: u32,
        on_page_change: Arc<dyn Fn(u32)>,
        navigate: impl Fn(&str, NavigateOptions) + Clone,
    ) {
        if page > 0 && page <= total_pages {
            current_page.set(page);
            on_page_change(page);
            let url = format!("rs-admin/posts/page/{}", page);
            let options = NavigateOptions::default();
            navigate(&url, options);
        }
    }

    view! {
        <nav aria-label="Page navigation">
            <ul class="pagination justify-content-center">
                <li class=move || {
                    if current_page.get() == 1 { "page-item disabled" } else { "page-item" }
                }>

                    {
                        let on_page_change_clone = on_page_change.clone();
                        let navigate_clone = navigate.clone();
                        view! {
                            <a
                                class="page-link"
                                href="#"
                                on:click=move |_| go_to_page(
                                    current_page.get() - 1,
                                    &current_page,
                                    total_pages.get(),
                                    on_page_change_clone.clone(),
                                    navigate_clone.clone(),
                                )
                            >

                                "Previous"
                            </a>
                        }
                    }

                </li>

                {
                    let total_pages_val = total_pages.get_untracked();
                    let current_page_val = current_page.get_untracked();
                    (1..=total_pages_val)
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
                                            &current_page,
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
                        .into_view()
                }

                <li class=move || {
                    if current_page.get() == total_pages.get() {
                        "page-item disabled"
                    } else {
                        "page-item"
                    }
                }>

                    {
                        let on_page_change_clone = on_page_change.clone();
                        let navigate_clone = navigate.clone();
                        view! {
                            <a
                                class="page-link"
                                href="#"
                                on:click=move |_| go_to_page(
                                    current_page.get() + 1,
                                    &current_page,
                                    total_pages.get(),
                                    on_page_change_clone.clone(),
                                    navigate_clone.clone(),
                                )
                            >

                                "Next"
                            </a>
                        }
                    }

                </li>
            </ul>
        </nav>
    }
}
