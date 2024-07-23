use leptos::*;

use crate::components::admin::header_content::HeaderContent;
use crate::utils::add_class::add_class;

#[component]
pub fn Dashboard() -> impl IntoView {
    add_class("body", "dashboard");

    view! { <HeaderContent title="Dashboard"/> }
}
