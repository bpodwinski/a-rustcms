use handlebars::Handlebars;
use leptos::*;
use std::path::Path;

use crate::utils::add_class::add_class;

#[component]
pub fn FrontHomeView() -> impl IntoView {
    add_class("body", "homepage");

    let mut handlebars = Handlebars::new();

    // Charger le template depuis ./themes/Default/index.hbs
    let template_path = Path::new("themes/Default/index.hbs");

    if let Err(e) = handlebars.register_template_file("index", &template_path) {
        eprintln!("Erreur lors du chargement du template : {:?}", e);
        return view! { <div>"Erreur lors du chargement du template."</div> };
    }

    let mut data = serde_json::Map::new();
    data.insert(
        "title".to_string(),
        serde_json::json!("Bienvenue sur mon blog"),
    );
    data.insert(
        "content".to_string(),
        serde_json::json!("Voici le contenu de votre article."),
    );

    // Rendre le template avec les données
    let rendered = match handlebars.render("index", &data) {
        Ok(rendered_html) => rendered_html,
        Err(e) => {
            eprintln!("Erreur lors du rendu du template : {:?}", e);
            return view! { <div>"Erreur lors du rendu du template."</div> };
        }
    };

    // Utilisation de inner_html pour injecter le rendu HTML dans Leptos
    view! { <div inner_html=rendered></div> }
}
