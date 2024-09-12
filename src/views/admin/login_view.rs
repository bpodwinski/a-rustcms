use ev::SubmitEvent;
use leptos::*;

use crate::components::admin::header_content::HeaderContent;
use crate::utils::add_class::add_class;

#[component]
pub fn LoginView() -> impl IntoView {
    add_class("body", "login");

    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        //let email_value = email.get();
        //let password_value = password.get();

        // Ajouter ici l'appel API ou la logique d'authentification
    };

    view! {
        <div class="container vh-100 d-flex align-items-center justify-content-center">
            <div class="row">
                <div class="col">

                    <HeaderContent title="Login"/>

                    <div class="card">
                        <div class="card-body">
                            <form on:submit=on_submit>

                                <div class="form-floating mb-3">
                                    <input
                                        type="email"
                                        class="form-control"
                                        id="email"
                                        name="email"
                                        prop.value=email
                                        on:input=move |ev| set_email.set(event_target_value(&ev))
                                        placeholder="Enter your email"
                                    />
                                    <label for="email">"Email"</label>
                                </div>

                                <div class="form-floating mb-3">
                                    <input
                                        type="password"
                                        class="form-control"
                                        id="password"
                                        name="password"
                                        prop.value=password
                                        on:input=move |ev| set_password.set(event_target_value(&ev))
                                        placeholder="Enter your password"
                                    />
                                    <label for="password">"Password"</label>
                                </div>

                                <button type="submit" class="btn btn-primary">"Login"</button>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
