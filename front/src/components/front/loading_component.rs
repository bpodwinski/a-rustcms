use leptos::*;

#[component]
pub fn LoadingComponent() -> impl IntoView {
    view! {
        <div class="loading-container container-fluid d-flex align-items-center justify-content-center">
            <div class="d-flex justify-content-center">
                <div class="spinner-border" style="width: 4rem; height: 4rem;" role="status">
                    <span class="visually-hidden">Loading...</span>
                </div>
            </div>
        </div>
    }
}
