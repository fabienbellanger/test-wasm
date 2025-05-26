//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use test_wasm::{add, greet};
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
