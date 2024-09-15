use leptos::*;
use leptos_router::*;

use crate::{
    components::admin::{admin_bar::AdminBar, admin_menu::AdminMenu},
    utils::add_class::add_class,
};

#[component]
pub fn AdminLayoutView() -> impl IntoView {
    add_class("body", "admin");

    view! {
        <div class="container-fluid">
            <div class="row">
                <div class="col">
                    <div class="wrapper content">

                        <AdminBar/>
                        <AdminMenu/>

                        <Outlet/>

                    </div>
                </div>
            </div>
        </div>
    }
}
