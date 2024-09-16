use leptos::*;

#[component]
pub fn ToastComponent(
    message: Signal<String>, // Signal pour lire le message du toast
    toast_type: Signal<String>, // Signal pour le type de toast (success, error)
    show: Signal<bool>,      // Signal pour afficher ou masquer le toast
    set_show: WriteSignal<bool>, // Setter pour afficher/masquer le toast
) -> impl IntoView {
    let toast_class = move || {
        if toast_type.get() == "success" {
            "toast align-items-center text-white bg-success border-0"
        } else {
            "toast align-items-center text-white bg-danger border-0"
        }
    };

    view! {
        <div class="position-fixed bottom-0 end-0 p-3" style="z-index: 5;">
            <div
                class=move || toast_class()
                role="alert"
                aria-live="assertive"
                aria-atomic="true"
                class:show=move || show.get()
            >
                <div class="d-flex">
                    <div class="toast-body">{move || message.get()}</div>
                    <button
                        type="button"
                        class="btn-close btn-close-white me-2 m-auto"
                        aria-label="Close"
                        on:click=move |_| set_show.set(false)
                    ></button>
                </div>
            </div>
        </div>
    }
}
