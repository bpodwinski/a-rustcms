pub mod app;
pub mod components;
pub mod constructors;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod models;
pub mod services;
pub mod utils;
pub mod views;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
