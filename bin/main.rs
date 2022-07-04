#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = shakespeare_monkey_solver::TemplateApp::default();
    eframe::run_native(
        Box::new(app),
        eframe::NativeOptions {
            ..eframe::NativeOptions::default()
        },
    );
}
