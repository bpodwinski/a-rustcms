use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::error_template::{AppError, ErrorTemplate};
use crate::views::admin::dashboard_view::AdminDashboardView;
use crate::views::admin::login_view::AdminLoginView;
use crate::views::admin::{
    layout_view::AdminLayoutView,
    posts::{post_new_view::AdminPostNew, posts_index_view::AdminPosts},
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let formatter = |text| format!("{text} - RustPress");

    view! {
        <Stylesheet id="leptos" href="/pkg/rustpress_view.css"/>
        <Stylesheet href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css"/>
        <Stylesheet href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.min.css"/>

        // JS
        <Script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Title formatter/>

                <Meta charset="utf-8"/>
                <Meta name="title" content="RustPress"/>
                <Meta name="description" content="A Wordpress clone in Rust!"/>

                <Html lang="en" dir="ltr" attr:data-bs-theme="dark"/>

                <div class="container-fluid">
                    <div class="row">
                        <div class="col">
                            <Routes>
                                <Routes>
                                    <Route path="/rs-admin" view=AdminLayoutView>
                                        <Route path="" view=AdminDashboardView/>
                                        <Route path="posts" view=AdminPosts/>
                                        <Route path="posts/new" view=AdminPostNew/>
                                    </Route>

                                    <Route path="/rs-admin/login" view=AdminLoginView/>

                                    <Route path="" view=|| view! { <p>"404 Not Found"</p> }/>
                                </Routes>
                            </Routes>
                        </div>
                    </div>
                </div>
            </main>
        </Router>
    }
}
