use leptos::*;
use leptos_router::A;

#[component]
pub fn AdminBar() -> impl IntoView {
    view! {
        <nav class="admin-bar navbar fixed-top navbar-expand-lg">
            <div class="container-fluid">
                <div class="collapse navbar-collapse" id="navbarNav">
                    <ul class="navbar-nav">
                        <li class="nav-item">
                            <A class="nav-link" href="/">
                                Visit site
                            </A>
                        </li>
                        <li class="nav-item">
                            <A class="nav-link" href="login">
                                Login
                            </A>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
