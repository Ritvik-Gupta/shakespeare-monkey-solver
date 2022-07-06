#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use shakespeare_monkey_solver::TemplateApp;

    #[cfg(feature = "puffin_profile")]
    start_puffin_server();

    eframe::run_native(
        "shakespeare-monkey-solver",
        eframe::NativeOptions {
            ..eframe::NativeOptions::default()
        },
        Box::new(|cc| {
            let mut app = TemplateApp::default();
            app.setup(cc);
            Box::new(app)
        }),
    );
}

#[cfg(feature = "puffin_profile")]
fn start_puffin_server() {
    puffin::set_scopes_on(true);

    match puffin_http::Server::new("0.0.0.0:8585") {
        Ok(puffin_server) => {
            eprintln!("Run:  cargo install puffin_viewer && puffin_viewer --url 127.0.0.1:8585");
            std::mem::forget(puffin_server);
        }
        Err(err) => eprintln!("Failed to start puffin server: {}", err),
    };
}
