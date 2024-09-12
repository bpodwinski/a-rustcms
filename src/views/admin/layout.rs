use leptos::*;
use leptos_router::*;

use crate::{
    components::admin::admin_bar::AdminBar, utils::add_class::add_class,
};

#[component]
pub fn AdminLayout() -> impl IntoView {
    add_class("body", "admin");

    view! {
        <AdminBar/>
        <nav class="admin-menu">
            <ul class="nav flex-column">
                <li class="nav-item">
                    <A class="nav-link" href="">
                        <i class="bi bi-speedometer2 me-2"></i>
                        Dashboard
                    </A>
                </li>
                <li class="nav-item">
                    <A class="nav-link" href="posts">
                        <i class="bi bi-pin-angle me-2"></i>
                        Posts
                    </A>
                </li>
            </ul>
        </nav>

        <Outlet/>
    }
}
