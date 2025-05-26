use std::sync::Once;
use wasm_bindgen::prelude::wasm_bindgen;

static SET_HOOK: Once = Once::new();

/// Sets a panic hook that logs panic messages to the console.
pub fn set_panic_hook() {
    SET_HOOK.call_once(|| {
        // When the `console_error_panic_hook` feature is enabled, we can call the
        // `set_panic_hook` function at least once during initialization, and then
        // we will get better error messages if our code ever panics.
        //
        // For more details see
        // https://github.com/rustwasm/console_error_panic_hook#readme
        //
        // Note that this is only enabled in debug builds, so it won't affect
        // performance in release builds.
        #[cfg(all(feature = "console_error_panic_hook", debug_assertions))]
        console_error_panic_hook::set_once();
    });
}

#[wasm_bindgen]
extern "C" {
    /// This function maps the `window.alert` function in JavaScript.
    #[wasm_bindgen(js_namespace = window, js_name = alert)]
    pub fn alert(s: &str);

    /// This function maps the `window.confirm` function in JavaScript.
    #[wasm_bindgen(js_namespace = window, js_name = confirm)]
    pub fn confirm(s: &str) -> bool;

    /// This function maps the `console.info` function in JavaScript.
    #[wasm_bindgen(js_namespace = console, js_name = info)]
    pub fn info(s: &str);

    /// This function maps the `console.log` function in JavaScript.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log(s: &str);

    /// This function maps the `console.info` function in JavaScript.
    #[wasm_bindgen(js_namespace = console, js_name = info)]
    pub fn warn(s: &str);

    /// This function maps the `console.error` function in JavaScript.
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn error(s: &str);

    /// This function maps the `window.print()` function in JavaScript.
    #[wasm_bindgen(js_namespace = window, js_name = print)]
    pub fn print();
}
