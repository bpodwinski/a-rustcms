use leptos::*;
use leptos_meta::*;

use crate::utils::add_class::add_class;

#[component]
pub fn Dashboard() -> impl IntoView {
    add_class("body", "dashboard");

    view! {
        <Title text="Dashboard"/>
        <h1 class="row">"Dashboard"</h1>
    }
}
