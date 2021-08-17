use wasm_bindgen::prelude::*;

// ===================================================================================================
// Demonstrates how to import functions from JS. Read more at 
// https://rustwasm.github.io/docs/wasm-bindgen/examples/import-js.html.
// You can read about all possible configuration options for importing JavaScript functions at 
// https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-js-imports/index.html.
// Note that you could deploy JS code alongside your Rust code (referenced .js files
// or inline JavaScript). Read more about this functionality at
// https://rustwasm.github.io/docs/wasm-bindgen/reference/js-snippets.html
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);      // Import built-in alert function
    fn myAlert(s: &str);    // Import global custom function
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);        // Import console.log function
}

// ===================================================================================================
// Demonstrates how to export functions from Rust.  You can read about all possible
// configuration options for exporting Rust functions at 
// https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/index.html.
#[wasm_bindgen(js_name = sayHello)]
pub fn say_hello() {
    alert("Hello World!");
}

#[wasm_bindgen(js_name = sayHelloWithMyAlert)]
pub fn say_hello_with_my_alert() {
    myAlert("Hello World!");
}

// ===================================================================================================
// Demonstrates how to handle numbers in parameters and return values.
// Note WASM-supported data types (https://webassembly.github.io/spec/core/appendix/index-types.html).
// Read more about number handling at
// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/numbers.html.
#[wasm_bindgen(js_name = workWithNumbers)]
pub fn work_with_numbers(x: f64, y: Option<f32>, r: i32) -> i64 {
    x as i64 + match y { Some(vy) => vy as i64, _ => 0i64 } + r as i64
}

#[wasm_bindgen]
pub fn run() {
    // Enable forwarding panic messages to console.error. Read more about this function at
    // https://github.com/rustwasm/console_error_panic_hook
    console_error_panic_hook::set_once();

    // Use console.log to display a status message. Note: UTF8 works 👍
    log("🦀 Rust WASM initialized 🤘")
}
