#![cfg_attr(not(debug_assertions), deny(warnings))]

mod app;

pub use app::TemplateApp;

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
pub mod web {
    use super::TemplateApp;
    use eframe::wasm_bindgen::{self, prelude::*};

    #[wasm_bindgen]
    pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();

        eframe::start_web(canvas_id, Box::new(|cc| Box::new(TemplateApp::new(cc))))
    }
}
