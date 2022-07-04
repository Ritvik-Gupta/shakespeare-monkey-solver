#![cfg_attr(not(debug_assertions), deny(warnings))]

mod app;
pub mod core;
pub mod utils;

pub use app::TemplateApp;

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
pub mod web {
    use super::TemplateApp;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
        // Make sure panics are logged using `console.error`.
        console_error_panic_hook::set_once();

        // Redirect tracing to console.log and friends:
        tracing_wasm::set_as_global_default();

        let app = TemplateApp::default();
        eframe::start_web(canvas_id, Box::new(app))
    }
}
