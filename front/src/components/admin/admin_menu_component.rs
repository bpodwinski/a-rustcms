use leptos::*;
use leptos_router::*;

#[component]
pub fn AdminMenu() -> impl IntoView {
    view! {
        <nav>
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
                <li class="nav-item">
                    <A class="nav-link" href="categories">
                        <i class="bi bi-bookmarks me-2"></i>
                        Categories
                    </A>
                </li>
                <li class="nav-item">
                    <A class="nav-link" href="tags">
                        <i class="bi bi-tag me-2"></i>
                        Tags
                    </A>
                </li>
            </ul>
        </nav>
    }
}
