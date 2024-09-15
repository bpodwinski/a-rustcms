use leptos::*;

use crate::{
    components::front::loading_component::LoadingComponent,
    services::admin::api::categories_api::get_categories,
};

#[component]
pub fn CategoriesComponent(
    categories_ids: Signal<Vec<u32>>, // Signal pour lire les catégories sélectionnées
    set_categories_ids: WriteSignal<Vec<u32>>, // Setter pour mettre à jour les catégories sélectionnées
) -> impl IntoView {
    // Utiliser create_resource pour récupérer les catégories via get_categories()
    let categories = create_resource(
        || (),
        |_| async {
            get_categories().await.ok() // Récupérer les catégories (ou None en cas d'erreur)
        },
    );

    let toggle_category = move |category_id: u32| {
        let current_ids = categories_ids.get();
        let mut new_ids = current_ids.clone();
        if new_ids.contains(&category_id) {
            // Si la catégorie est déjà sélectionnée, on la retire
            new_ids.retain(|&id| id != category_id);
        } else {
            // Sinon, on l'ajoute à la sélection
            new_ids.push(category_id);
        }
        set_categories_ids.set(new_ids); // Mettre à jour les catégories sélectionnées
    };

    view! {
        <div class="categories-component mb-3">
            <h3 for="post-categories" class="form-label">
                Categories
            </h3>
            <Suspense fallback=move || {
                view! { <LoadingComponent/> }
            }>
                // Affichage en fonction des données récupérées
                {move || match categories.get() {
                    Some(Some(categories)) => {
                        view! {
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
                        }
                    }
                    Some(None) => view! { <div>"Aucune catégorie trouvée."</div> },
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
    }
}
