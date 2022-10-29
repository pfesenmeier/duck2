//! trunk only lets main.rs, not any binary
//!
//!
use duck2::*;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch(app);
}
