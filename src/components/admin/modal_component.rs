use leptos::*;

/// Defines the size of the modal
pub enum Size {
    Small,
    Large,
    ExtraLarge,
    Default,
}
/// Defines the fullscreen behavior of the modal
pub enum Fullscreen {
    Always,
    SmallDown,
    MediumDown,
    LargeDown,
    ExtraLargeDown,
    ExtraExtraLargeDown,
    None,
}

/// Defines the vertical alignment of the modal
pub enum VerticalAlignment {
    Centered,
    CenteredScrollable,
    Default,
}

/// Defines the heading level of the modal title
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

/// A customizable modal component for displaying content in a dialog.
///
/// # Parameters
///
/// - `id`: A unique identifier for the modal (required).
/// - `toggle`: A signal to control the modal's visibility (open/close) (required).
/// - `title`: The title displayed in the modal header (required).
/// - `body`: The main content displayed in the modal body (required).
/// - `footer`: An optional footer for displaying buttons or actions (optional).
/// - `size`: Controls the size of the modal (small, large, extra-large, or default). Default is `Size::Default`.
/// - `fullscreen`: Defines the fullscreen behavior based on screen sizes. Default is `Fullscreen::None`.
/// - `animation`: Specifies whether the modal should animate when opening and closing. Default is `true`.
/// - `vertical_alignment`: Specifies the vertical alignment of the modal (centered or scrollable). Default is `VerticalAlignment::Default`.
/// - `scrollable`: Enables scrolling within the modal's body if the content is too large. Default is `false`.
/// - `static_backdrop`: Prevents the modal from closing when clicking outside the modal. Default is `false`.
/// - `heading_level`: Sets the HTML heading level (h1 to h6) for the modal's title. Default is `HeadingLevel::H1`.
/// - `css_class`: Adds a CSS class to customize the appearance of the modal. Default is an empty string.
#[component]
pub fn Modal(
    id: String,
    toggle: RwSignal<bool>,
    title: String,
    body: impl IntoView + 'static,
    footer: Option<View>,
    #[prop(default = Size::Default)] size: Size,
    #[prop(default = Fullscreen::None)] fullscreen: Fullscreen,
    #[prop(default = true)] animation: bool,
    #[prop(default = VerticalAlignment::Default)] vertical_alignment: VerticalAlignment,
    #[prop(default = false)] scrollable: bool,
    #[prop(default = false)] static_backdrop: bool,
    #[prop(default = HeadingLevel::H1)] heading_level: HeadingLevel,
    #[prop(default = "".to_string())] css_class: String,
) -> impl IntoView {
    let close_modal = move || toggle.set(false);

    let animation_class = if animation { "fade" } else { "" };
    let scrollable_class = if scrollable {
        "modal-dialog-scrollable"
    } else {
        ""
    };
    let static_backdrop_class = if static_backdrop { "static" } else { "true" };

    let size_class = match size {
        Size::Small => "modal-sm",
        Size::Large => "modal-lg",
        Size::ExtraLarge => "modal-xl",
        Size::Default => "",
    };

    let fullscreen_class = match fullscreen {
        Fullscreen::Always => "modal-fullscreen",
        Fullscreen::SmallDown => "modal-fullscreen-sm-down",
        Fullscreen::MediumDown => "modal-fullscreen-md-down",
        Fullscreen::LargeDown => "modal-fullscreen-lg-down",
        Fullscreen::ExtraLargeDown => "modal-fullscreen-xl-down",
        Fullscreen::ExtraExtraLargeDown => "modal-fullscreen-xxl-down",
        Fullscreen::None => "",
    };

    let vertical_alignment_class = match vertical_alignment {
        VerticalAlignment::Centered => "modal-dialog-centered",
        VerticalAlignment::CenteredScrollable => "modal-dialog-centered modal-dialog-scrollable",
        VerticalAlignment::Default => "",
    };

    let render_title = move || -> View {
        match heading_level {
            HeadingLevel::H1 => view! { <h1>{title.clone()}</h1> }.into_view(),
            HeadingLevel::H2 => view! { <h2>{title.clone()}</h2> }.into_view(),
            HeadingLevel::H3 => view! { <h3>{title.clone()}</h3> }.into_view(),
            HeadingLevel::H4 => view! { <h4>{title.clone()}</h4> }.into_view(),
            HeadingLevel::H5 => view! { <h5>{title.clone()}</h5> }.into_view(),
            HeadingLevel::H6 => view! { <h6>{title.clone()}</h6> }.into_view(),
        }
    };

    let css_classes = format!("modal {} {}", animation_class, css_class);

    view! {
        <div
            class=move || css_classes.clone()

            id=id.clone()
            tabindex="-1"
            role="dialog"
            aria-labelledby="modalLabel"
            aria-hidden=move || (!toggle.get()).to_string()
            data-bs-backdrop=static_backdrop_class
            data-bs-keyboard=static_backdrop_class
        >
            <div class=move || {
                format!(
                    "modal-dialog {} {} {} {}",
                    size_class,
                    fullscreen_class,
                    vertical_alignment_class,
                    scrollable_class,
                )
            }>
                <div class="modal-content">

                    <div class="modal-header">
                        {render_title()}
                        <button
                            type="button"
                            class="btn-close"
                            data-bs-dismiss="modal"
                            on:click=move |_| close_modal()
                            aria-label="Close"
                        ></button>
                    </div>

                    <div class="modal-body">{body.into_view()}</div>

                    {footer.map(|f| view! { <div class="modal-footer">{f.into_view()}</div> })}

                </div>
            </div>
        </div>
    }
}
