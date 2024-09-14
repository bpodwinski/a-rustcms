use handlebars::Handlebars;
use leptos::*;
use serde_json::Value;
use std::collections::HashMap;

use crate::services::admin::api::posts::get_posts;

#[component]
pub fn FrontHomeView() -> impl IntoView {
    // Créer une ressource pour récupérer les posts de manière asynchrone
    let posts = create_resource(|| (), |_| async { get_posts().await });

    view! {
        <Suspense fallback=move || {
            view! { <p>"Chargement des posts..."</p> }
        }>
            {move || {
                match posts.get() {
                    Some(Ok(posts)) => {
                        let mut handlebars = Handlebars::new();
                        let template = include_str!("../../themes/Default/home.hbs");
                        handlebars
                            .register_template_string("home", template)
                            .expect("Template string could not be loaded");
                        let mut data = HashMap::new();
                        data.insert(
                            "title".to_string(),
                            Value::String("Bienvenue sur la Homepage".to_string()),
                        );
                        let posts_for_template: Vec<HashMap<String, String>> = posts
                            .into_iter()
                            .map(|post| {
                                let mut post_map = HashMap::new();
                                post_map.insert("title".to_string(), post.title);
                                post_map.insert("content".to_string(), post.content);
                                post_map
                            })
                            .collect();
                        let posts_json = match serde_json::to_value(&posts_for_template) {
                            Ok(json) => json,
                            Err(e) => {
                                return view! {
                                    <div>
                                        <p>
                                            {format!("Erreur de conversion des posts en JSON: {}", e)}
                                        </p>
                                    </div>
                                };
                            }
                        };
                        data.insert("posts".to_string(), posts_json);
                        let html = match handlebars.render("home", &data) {
                            Ok(html) => html,
                            Err(e) => {
                                return view! {
                                    <div>
                                        <p>{format!("Erreur lors du rendu du template: {}", e)}</p>
                                    </div>
                                };
                            }
                        };
                        view! { <div inner_html=html></div> }
                    }
                    Some(Err(err)) => {
                        view! {
                            <div>
                                <p>{format!("Erreur lors du chargement des posts: {}", err)}</p>
                            </div>
                        }
                    }
                    None => {
                        view! {
                            <div>
                                <p>"Chargement..."</p>
                            </div>
                        }
                    }
                }
            }}

        </Suspense>
    }
}
