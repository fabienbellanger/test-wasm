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

#[cfg(test)]
mod tests {
    #![cfg(target_arch = "wasm32")]

    use super::*;
    use wasm_bindgen_test::*;

    extern crate wasm_bindgen_test;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_greet() {
        assert_eq!(greet("John"), String::from("Hello, John!"));
        assert_eq!(greet(""), String::from("Hello!"));
    }

    #[wasm_bindgen_test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
