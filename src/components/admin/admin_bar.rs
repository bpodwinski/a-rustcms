use leptos::*;
use leptos_router::A;

#[component]
pub fn AdminBar() -> impl IntoView {
    view! {
        <div class="admin-bar">
            <ul>
                <li class="login">
                    <A href="login">Login</A>
                </li>
            </ul>
        </div>
    }
}
