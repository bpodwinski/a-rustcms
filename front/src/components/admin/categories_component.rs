use leptos::*;

use crate::{
    components::front::loading_component::LoadingComponent,
    services::admin::api::categories_api::get_categories,
};

#[component]
pub fn CategoriesComponent(
    categories_ids: Signal<Vec<u32>>,
    set_categories_ids: WriteSignal<Vec<u32>>,
) -> impl IntoView {
    let categories =
        create_resource(|| (), |_| async { get_categories().await.ok() });

    let toggle_category = move |category_id: u32| {
        let current_ids = categories_ids.get();
        let mut new_ids = current_ids.clone();
        if new_ids.contains(&category_id) {
            new_ids.retain(|&id| id != category_id);
        } else {
            new_ids.push(category_id);
        }
        set_categories_ids.set(new_ids);
    };

    view! {
        <div class="categories-component accordion mb-5" id="categories-component">
            <div class="accordion-item">

                <div class="accordion-header">
                    <button
                        class="accordion-button"
                        type="button"
                        data-bs-toggle="collapse"
                        data-bs-target="#collapseCategories"
                        aria-expanded="true"
                        aria-controls="collapseCategories"
                    >
                        <h3>Categories</h3>
                    </button>
                </div>

                <div
                    id="collapseCategories"
                    class="accordion-collapse collapse show"
                    data-bs-parent="#categories-component"
                >
                    <div class="accordion-body">

                        <Suspense fallback=move || {
                            view! {
                                <div>
                                    <LoadingComponent/>
                                </div>
                            }
                        }>
                            {move || match categories.get() {
                                Some(Some(categories)) => {
                                    view! {
                                        <div class="card-body">
                                            <div class="form-check-group">
                                                {categories
                                                    .iter()
                                                    .map(|category| {
                                                        let category_id = category.id;
                                                        let category_name = category.name.clone();
                                                        let is_checked = categories_ids
                                                            .get()
                                                            .contains(&category_id);
                                                        view! {
                                                            <div class="form-check">
                                                                <input
                                                                    class="form-check-input"
                                                                    type="checkbox"
                                                                    id=category.id.clone()
                                                                    on:change=move |_| toggle_category(category_id)
                                                                    prop:checked=is_checked
                                                                />
                                                                <label class="form-check-label" for=category.id.clone()>
                                                                    {category_name}
                                                                </label>
                                                            </div>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </div>
                                        </div>
                                    }
                                }
                                Some(None) => {
                                    view! {
                                        <div>
                                            <p class="text-center">
                                                "An issue occurred while retrieving the categories"
                                            </p>
                                        </div>
                                    }
                                }
                                None => {
                                    view! {
                                        <div>
                                            <LoadingComponent/>
                                        </div>
                                    }
                                }
                            }}

                        </Suspense>
                    </div>
                </div>
            </div>
        </div>
    }
}
