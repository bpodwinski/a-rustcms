use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::error_template::{AppError, ErrorTemplate};
use crate::views::admin::dashboard::dashboard_view::AdminDashboardView;
use crate::views::admin::layout_view::AdminLayoutView;
use crate::views::admin::login::login_view::AdminLoginView;
use crate::views::admin::posts::post_new_view::AdminPostNewView;
use crate::views::admin::posts::posts_index_view::AdminPostsView;
use crate::views::admin::tags::tags_index_view::AdminTagsView;
use crate::views::front::home_view::FrontHomeView;

#[component]
pub fn App() -> impl IntoView {
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

                                // Admin routes
                                <Route path="/rs-admin" view=AdminLayoutView>
                                    <Route path="" view=AdminDashboardView/>

                                    // Posts routes
                                    <Route path="posts" view=AdminPostsView/>
                                    <Route path="posts/new" view=AdminPostNewView/>

                                    // Tags routes
                                    <Route path="tags" view=AdminTagsView/>
                                </Route>

                                <Route path="/rs-admin/login" view=AdminLoginView/>

                                // Front routes
                                <Route path="/" view=FrontHomeView/>

                            </Routes>
                        </div>
                    </div>
                </div>
            </main>
        </Router>
    }
}
