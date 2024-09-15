use leptos::*;

use crate::components::admin::header_content_component::HeaderContent;
use crate::utils::add_class_util::add_class;

#[component]
pub fn AdminDashboardView() -> impl IntoView {
    add_class("body", "dashboard");

    view! { <HeaderContent title="Dashboard"/> }
}
