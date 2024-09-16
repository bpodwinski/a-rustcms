use leptos::*;

use crate::{
    components::admin::notification_component::ToastComponent,
    constructors::admin_new_content_view::AdminNewContentView,
    utils::add_class_util::add_class,
};

#[component]
pub fn AdminPostNewView() -> impl IntoView {
    add_class("body", "post-new");

    // Utilisation de ContentView pour les signaux
    let content_view = AdminNewContentView::new_post();

    // Signaux pour les notifications (gérés localement)
    let (notification_message, set_notification_message) =
        create_signal(String::new());
    let (notification_type, set_notification_type) =
        create_signal(String::new());
    let (show_toast, set_show_toast) = create_signal(false);

    view! {
        <ToastComponent
            message=notification_message.into()
            toast_type=notification_type.into()
            show=show_toast.into()
            set_show=set_show_toast
        />

        {content_view
            .render(
                content_view.title.write_only(),
                content_view.content.write_only(),
                content_view.categories_ids.write_only(),
                content_view.status.write_only(),
                content_view.date_published.write_only(),
                set_notification_message,
                set_notification_type,
                set_show_toast,
            )}
    }
}
