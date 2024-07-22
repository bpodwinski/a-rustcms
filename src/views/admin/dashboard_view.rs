use leptos::*;
use leptos_meta::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    create_effect(move |_| {
        let body = document().body().expect("document should have a body");
        body.class_list()
            .add_1("dashboard")
            .expect("could not add class");
        on_cleanup(move || {
            body.class_list()
                .remove_1("dashboard")
                .expect("could not remove class");
        });
    });

    view! {
        <Title text="Dashboard"/>
        <h1>"Dashboard"</h1>
    }
}
