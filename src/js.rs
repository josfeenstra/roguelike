use wasm_bindgen::prelude::*;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// // Export a `greet` function from Rust to JavaScript, that alerts a
// // hello message.
pub fn print(msg: &str) {
    log(msg);
}