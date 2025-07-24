use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

mod models;
mod components;

use components::Dashboard;

// Point d'entrée pour WebAssembly
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    console_error_panic_hook::set_once();
    
    dioxus::launch(app);
}

// Point d'entrée standard Rust
fn main() {
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" }
        document::Link { rel: "stylesheet", href: "assets/styles.css" }

        Dashboard {}
    }
}

// Export alternatif pour compatibilité
#[wasm_bindgen]
pub fn start_app() {
    wasm_main();
}