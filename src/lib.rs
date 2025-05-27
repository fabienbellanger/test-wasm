pub mod task;
pub mod utils;

use utils::{log, set_panic_hook};
use wasm_bindgen::prelude::*;

/// This function is called when the WebAssembly module is initialized.
#[wasm_bindgen]
pub fn setup() {
    log("Setting up test-wasm...");
    set_panic_hook();
}

#[wasm_bindgen]
pub fn greet(value: &str) -> String {
    if value.is_empty() {
        return "Hello!".to_string();
    }
    format!("Hello, {}!", value)
}

#[wasm_bindgen]
pub fn add(a: isize, b: isize) -> isize {
    a + b
}
