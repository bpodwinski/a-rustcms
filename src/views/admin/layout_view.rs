use leptos::*;
use leptos_router::*;

use crate::{
    components::admin::{
        admin_bar_component::AdminBar, admin_menu_component::AdminMenu,
    },
    utils::add_class_util::add_class,
};

#[component]
pub fn AdminLayoutView() -> impl IntoView {
    add_class("body", "admin");

    view! {
        <div class="container-fluid">
            <div class="row">
                <div class="col-auto admin-menu">
                    <AdminMenu/>
                </div>
                <div class="col admin-content">
                    <AdminBar/>
                    <div class="wrapper mt-3">
                        <Outlet/>
                    </div>
                </div>
            </div>
        </div>
    }
}
