use leptos::{create_effect, document, on_cleanup};

/// Adds a CSS class to an HTML element specified by a CSS selector.
/// The class will be added when the effect is created and removed when the effect is cleaned up.
///
/// # Arguments
///
/// * `selector` - A CSS selector that identifies the target HTML element.
/// * `class_name` - The name of the CSS class to add to the element.
///
/// # Example
///
/// ```
/// add_class("#my-element", "active");
/// ```
///
/// # Errors
///
/// This function will print error messages to the console if:
/// - The element cannot be found using the provided selector.
/// - The class cannot be added or removed from the element.
pub fn add_class(selector: &str, class_name: &str) {
    let selector = selector.to_string();
    let class_name = class_name.to_string();

    create_effect(move |_| match document().query_selector(&selector) {
        Ok(Some(element)) => {
            if let Err(err) = element.class_list().add_1(&class_name) {
                eprintln!("Failed to add class '{}': {:?}", class_name, err);
            }

            on_cleanup({
                let class_name = class_name.clone();
                let selector = selector.clone();
                move || match document().query_selector(&selector) {
                    Ok(Some(element)) => {
                        if let Err(err) = element.class_list().remove_1(&class_name) {
                            eprintln!("Failed to remove class '{}': {:?}", class_name, err);
                        }
                    }
                    Ok(None) => eprintln!("Element not found for selector '{}'", selector),
                    Err(err) => eprintln!("Failed to query selector '{}': {:?}", selector, err),
                }
            });
        }
        Ok(None) => eprintln!("Element not found for selector '{}'", selector),
        Err(err) => eprintln!("Failed to query selector '{}': {:?}", selector, err),
    });
}
