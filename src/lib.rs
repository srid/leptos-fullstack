mod app;
mod thing;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(not(feature = "ssr"))]
#[wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    use leptos::*;
    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(move |cx| {
        view! { cx, <App/> }
    });
}
