mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() -> String{
    set_panic_hook();
    
    alert("Hello, test-wasm!");
    String::from("Hello, test-wasm!")
}
