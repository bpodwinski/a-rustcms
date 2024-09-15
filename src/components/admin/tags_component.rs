use leptos::*;

use crate::{
    components::front::loading_component::LoadingComponent,
    services::admin::api::tags_api::get_tags,
};

#[component]
pub fn TagsComponent(
    tags_ids: Signal<Vec<u32>>, // Signal pour lire les catégories sélectionnées
    set_tags_ids: WriteSignal<Vec<u32>>, // Setter pour mettre à jour les catégories sélectionnées
) -> impl IntoView {
    // Utiliser create_resource pour récupérer les catégories via get_tags()
    let tags = create_resource(
        || (),
        |_| async {
            get_tags().await.ok() // Récupérer les catégories (ou None en cas d'erreur)
        },
    );

    let toggle_tag = move |tag_id: u32| {
        let current_ids = tags_ids.get();
        let mut new_ids = current_ids.clone();
        if new_ids.contains(&tag_id) {
            // Si la catégorie est déjà sélectionnée, on la retire
            new_ids.retain(|&id| id != tag_id);
        } else {
            // Sinon, on l'ajoute à la sélection
            new_ids.push(tag_id);
        }
        set_tags_ids.set(new_ids); // Mettre à jour les catégories sélectionnées
    };

    view! {
        <div class="tags-component mb-3">
            <h3 for="post-tags" class="form-label">
                tags
            </h3>
            <Suspense fallback=move || {
                view! { <LoadingComponent/> }
            }>
                // Affichage en fonction des données récupérées
                {move || match tags.get() {
                    Some(Some(tags)) => {
                        view! {
                            <div class="form-check-group">
                                {tags
                                    .iter()
                                    .map(|tag| {
                                        let tag_id = tag.id;
                                        let tag_name = tag.name.clone();
                                        let is_checked = tags_ids.get().contains(&tag_id);
                                        view! {
                                            <div class="form-check">
                                                <input
                                                    class="form-check-input"
                                                    type="checkbox"
                                                    id=tag.id.clone()
                                                    on:change=move |_| toggle_tag(tag_id)
                                                    prop:checked=is_checked
                                                />
                                                <label class="form-check-label" for=tag.id.clone()>
                                                    {tag_name}
                                                </label>
                                            </div>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        }
                    }
                    Some(None) => view! { <div>"Aucun tags trouvé."</div> },
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
