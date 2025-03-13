use leptos::*;
use leptos_meta::*;

pub struct ButtonProps<'a> {
    pub text: &'a str,
    pub url: &'a str,
}

/// HeaderContent component.
///
/// This component displays a header with a title and, optionally, a button.
/// The button is optional and can be omitted by not providing a value for the `button` argument.
///
/// # Arguments
///
/// * `title` - The title to display in the header.
/// * `button` - An option containing the properties of the button (text and URL).
///
/// # Example
///
/// ```rust
/// use crate::components::admin::header_content::{HeaderContent, ButtonProps};
///
/// HeaderContent(
///     title="Dashboard",
///     button=ButtonProps { text: "Add new post", url: "/rs-admin/posts/new" }
/// );
/// ```
///
/// # Example without button
///
/// ```rust
/// use crate::components::admin::header_content::HeaderContent;
///
/// HeaderContent(title="Dashboard", button=None);
/// ```
#[component]
pub fn HeaderContent<'a>(
    title: &'a str,
    #[prop(optional)] button: Option<ButtonProps<'a>>,
) -> impl IntoView {
    let title = title.to_string();
    let button_props = button.map(|b| (b.text.to_string(), b.url.to_string()));

    view! {
        <div class="header-content">
            <Title text=title.clone()/>
            <h1>{title}</h1>
            {move || {
                if let Some((text, url)) = &button_props {
                    view! {
                        <a class="btn btn-outline-primary" href=url.clone() role="button">
                            {text.clone()}
                        </a>
                    }
                        .into_view()
                } else {
                    view! { <></> }.into_view()
                }
            }}

        </div>
    }
}
