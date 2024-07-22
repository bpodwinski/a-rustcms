use crate::error_template::{AppError, ErrorTemplate};
use crate::views::{
    admin::dashboard_view::Dashboard, admin::posts::post_new_view::PostNew,
    admin::posts::posts_index_view::Posts,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let formatter = |text| format!("{text} - RustPress");

    view! {


        // CSS
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rustpress_view.css"/>
        <Stylesheet href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css"/>
        <Stylesheet href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.min.css"/>

        // JS
        <Script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <nav class="admin-menu">
                <ul class="nav flex-column">
                  <li class="nav-item">
                    <a class="nav-link" href="/rs-admin">
                        <i class="bi bi-speedometer2 me-2"></i>
                        Dashboard
                    </a>
                  </li>
                  <li class="nav-item">
                    <a class="nav-link" href="/rs-admin/posts">
                        <i class="bi bi-pin-angle me-2"></i>
                        Posts
                    </a>
                  </li>
                </ul>
            </nav>
            <main>
                <Title formatter/>

                <Meta charset="utf-8"/>
                <Meta name="title" content="RustPress"/>
                <Meta name="description" content="A Wordpress clone in Rust!"/>

                <Html
                  lang="en"
                  dir="ltr"
                  attr:data-theme="light"
                />
                    <div class="container-fluid">
                        <div class="row">
                            <div class="col">
                                <Routes>
                                    <Route path="/rs-admin" view=Dashboard/>
                                    <Route path="/rs-admin/posts" view=Posts/>
                                        <Route path="/rs-admin/posts/new" view=PostNew/>
                                </Routes>
                            </div>
                        </div>
                    </div>
            </main>
        </Router>
    }
}
