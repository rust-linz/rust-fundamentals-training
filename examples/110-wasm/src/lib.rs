use wasm_bindgen::{prelude::*};
use serde::{Serialize, Deserialize};
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlLiElement, HtmlUListElement};
use wasm_bindgen::JsCast;

// region: Initializing
#[wasm_bindgen(start)]
pub fn run() {
    // Enable forwarding panic messages to console.error. Read more about this function at
    // https://github.com/rustwasm/console_error_panic_hook. 
    // If you want to really optimize your Wasm size to the very last bit, avoid
    // panicing (read more e.g. at https://rustwasm.github.io/book/reference/code-size.html#avoid-panicking).
    console_error_panic_hook::set_once();

    // Use console.log to display a status message. Note: UTF8 works 👍
    log("🦀 Rust Wasm initialized 🤘".to_string());
}
// endregion: Initializing

// region: Importing/exporting functions from JS/Rust
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
    fn log(s: String);        // Import console.log function
}

// ===================================================================================================
// Demonstrates how to export functions from Rust.  You can read about all possible
// configuration options for exporting Rust functions at 
// https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/index.html.

/// Says hello using JavaScript's `alert` function
#[wasm_bindgen(js_name = sayHello)]
pub fn say_hello() {
    alert("Hello World!");
}

/// Says hello using custom `myAlert` function
#[wasm_bindgen(js_name = sayHelloWithMyAlert)]
pub fn say_hello_with_my_alert() {
    myAlert("Hello World!");
}
// endregion: Importing/exporting functions from JS/Rust

// region: Handle parameters
// ===================================================================================================
// Demonstrates how to handle numbers in parameters and return values.
// Note Wasm-supported data types (https://webassembly.github.io/spec/core/appendix/index-types.html).
// Read more about number handling at
// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/numbers.html.

/// Adds three numbers
///
/// * `x` - A float number
/// * `y` - An optional float number
/// * `r` - An int number
#[wasm_bindgen(js_name = workWithNumbers)]
pub fn work_with_numbers(x: f64, y: Option<f32>, r: i32) -> i64 {
    x as i64 + match y { Some(vy) => vy as i64, _ => 0i64 } + r as i64
}

// ===================================================================================================
// Demonstrates how to handle booleans in parameters and return values.
// Read more about bool handling at
// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/bool.html.

/// Checks whether all parameters are true
#[wasm_bindgen(js_name = allTrue)]
pub fn all_true(a: bool, another: bool) -> bool {
    a && another
}
// endregion: Handle parameters

// region: Panic
// ===================================================================================================
// Demonstrates the consequences of using console_error_panic_hook.
// Try panic with and without console_error_panic_hook and see the difference in the browser's console log.

/// Will panic
#[wasm_bindgen]
#[allow(unconditional_panic)]
pub fn panic() -> i32 {
    let x = 42;
    let y = 0;
    x / y
}
// endregion: Panic

// region: Number slices
// ===================================================================================================
// Demonstrates how to work with number slices. Read more about number slices at
// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/number-slices.html and
// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/boxed-number-slices.html.

/// Fills given number slice with fibonacci numbers
#[wasm_bindgen(js_name = fillWithFibonacci)]
pub fn fill_with_fibonacci(buffer: &mut [i32]) {
    if buffer.is_empty() { return; }
    buffer[0] = 0;

    if buffer.len() == 1 { return; }
    buffer[1] = 1;

    for i in 2..buffer.len() {
        buffer[i] = buffer[i - 1] + buffer[ i - 2];
    }
}
// endregion: Number slices

// region: Strings
// ===================================================================================================
// Demonstrates how to work with strings. Read more about handling strings at
// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/str.html,
// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/string.html, and
// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/char.html.

/// Prints the given message to console and returns the message unchanged.
#[wasm_bindgen(js_name = writeToConsole)]
pub fn write_to_console(msg: &str) -> String {
    log(msg.to_string());
    msg.to_string()
}
// endregion: Strings

// region: Exported types
// ===================================================================================================
// Demonstrates how to work with exported types. Check the generated .d.ts file for the related
// TypeScript type definitions. For more information about Rust-exported types see
// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/exported-rust-types.html.
// Note that we need to implement a static constructor function. The `new` keyword
// cannot be used in JS in conjunction with `Person`. Also note that String props 
// cannot be made public because of the missing Copy trait. We need to explicitly provide getters.

/// Represents a person
#[wasm_bindgen]
pub struct Person {
    first_name: String,
    last_name: String,
    pub age: f64,
}

#[wasm_bindgen]
impl Person {
    /// Constructs a new person.
    #[wasm_bindgen]
    pub fn new(first_name: String, last_name: String, age: f64) -> Person {
        Person { first_name, last_name, age}
    }

    #[wasm_bindgen(getter)]
    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }
}

// For demo purposes, we implement the Drop trait manually. This can be used to find out
// if instances are dropped properly.
impl Drop for Person {
    fn drop(&mut self) {
        log(format!("Dropping {}", self.first_name));
    }
}

/// Adds given change to a person's age and returns the changed person.
#[wasm_bindgen(js_name = "fixAge")]
pub fn fix_age(person: Person, age_change: f64) -> Person {
    // Note: If you would change person to &Person instead of Person, the function
    // would no longer take ownership and the person would not be dropped automatically. If we take
    // a borrowed reference, JS is responsible to explicitly call `free`.
    Person { first_name: "Rust".to_string(), last_name: person.last_name.clone(), age: person.age + age_change }
}
// endregion: Exported types

// region: Working with JsValue
// ===================================================================================================
// Demonstrates how to work with JsValue type. For more information see
// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/jsvalue.html.

#[derive(Serialize, Deserialize)]
struct PersonDynamic {
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    age: f64,
}

// Turns any given std::error::Error into a JS error message
fn into_js_error(err: impl std::error::Error) -> JsValue {
    // Note that we could just return err.to_string().into().
    // However, in that case we would not get the stack trace.
    js_sys::Error::new(&err.to_string()).into()
}

// Note that the use of JsValue results in `any` in TypeScript.
#[wasm_bindgen(js_name = "fixAgeDynamic")]
pub fn fix_age_dynamic(person: JsValue, age_change: JsValue) -> Result<JsValue, JsValue> {
    // Serialize data in JS using JSON.stringify, send string to Rust, and deserialize 
    // everything in Rust with serde. Relatively expensive, but good compatibility.
    // Note that we return a meaningful error if deserilization does not work.
    let mut p: PersonDynamic = serde_wasm_bindgen::from_value(person).map_err(into_js_error)?;

    // Check data type and process it if data type fits
    if let Some(ac) = age_change.as_f64() {
        p.age += ac;
    }

    // Serialize result in Rust, send string to JS, and deserialize everything in JS.
    let result = serde_wasm_bindgen::to_value(&p).map_err(into_js_error)?;
    Ok(result)
}
// endregion: Working with JsValue

// region: Working with serde_wasm_bindgen
// ===================================================================================================
// Demonstrates how to work with serde_wasm_bindgen. For more information see
// https://crates.io/crates/serde-wasm-bindgen. The big difference compared to the 
// sample above is that data is not serialized/deserialized via JSON. The crate
// uses direct APIs for JavaScript value manipulation instead.

#[wasm_bindgen(js_name = "fixAgeSerdeWasm")]
pub fn fix_age_serde_wasm(person: JsValue, age_change: JsValue) -> Result<JsValue, JsValue> {
    let mut p: PersonDynamic = serde_wasm_bindgen::from_value(person)?;
    let ac: f64 = serde_wasm_bindgen::from_value(age_change)?;
    p.age += ac;
    Ok(serde_wasm_bindgen::to_value(&p)?)
}
// endregion: Working with serde_wasm_bindgen

// region: Working with shared buffers
// ===================================================================================================
// Demonstrates how to work with a shared buffer between Rust and JS. For more information see
// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/pointers.html.
// In this example we implement Knight Rider-like scanner lights.
#[derive(PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[wasm_bindgen]
pub struct Display {
    pixel: Vec<u8>,
    direction: Direction,
}

/// Intensity of leading light
const MAX_INTENSITY: u8 = 5;

#[wasm_bindgen]
impl Display {
    /// Creates a new "display". Each display represents the light intensity of the corresponding LED.
    pub fn new(size: usize) -> Display {
        let mut result = Display { pixel: vec![0; size], direction: Direction::Right, };
        result.pixel[0] = MAX_INTENSITY;
        result
    }

    /// Gets a pointer to the shared buffer with light intensities per LED.
    pub fn pixel(&self) -> *const u8 {
        self.pixel.as_ptr()
    }

    /// Advance LED to next position.
    pub fn next(&mut self) {
        let ix = self.pixel.iter().position(|p| *p == MAX_INTENSITY).unwrap();
        
        if ix == 0 {
            // We reached left border -> turn around
            self.direction = Direction::Right;
        }
        else if ix == self.pixel.len() - 1 { 
            // We reached right border -> turn around
            self.direction = Direction::Left;
        }

        // Calculate next index of leading light
        let next_ix: usize = ix + match self.direction { Direction::Left => -1, Direction::Right => 1 } as usize;

        // Reduce intensity of remaining lights
        self.pixel.iter_mut().for_each(|p| if *p > 0 { *p -= 1; });

        // Set leading light
        self.pixel[next_ix] = MAX_INTENSITY;
    }
}
// endregion: Working with shared buffers

// region: Manipulate DOM from Rust using web_sys
#[wasm_bindgen]
pub fn setup_todo() {
    let mut todo_items = Vec::<String>::new();

    // Get a reference to the window
    let window = web_sys::window().unwrap();
    
    // Get element references to input and button element
    let document = window.document().unwrap();
    let new_todo_item_element = document.get_element_by_id("newTodoItem").unwrap();
    let todo_items_element = document.get_element_by_id("todoItems").unwrap();
    
    // Define JS-callable closure for button handler
    let btn_handler = Closure::wrap(Box::new(move || {
        // Cast element reference to proper types
        let new_todo_item = new_todo_item_element.dyn_ref::<HtmlInputElement>().unwrap();
        let todo_items_list = todo_items_element.dyn_ref::<HtmlUListElement>().unwrap();

        // Add new todo item to vector
        todo_items.push(new_todo_item.value());

        // Fill ulist with elements from todo_list
        todo_items_list.set_inner_text("");
        for item in todo_items.iter() {
            // Create li element
            let new_li_element = document.create_element("li").unwrap();
            let new_li = new_li_element.dyn_ref::<HtmlLiElement>().unwrap();

            // Set text content of li element
            new_li.set_inner_text(item.as_str());

            // Append li element
            todo_items_list.append_child(new_li).unwrap();
        }

    }) as Box<dyn FnMut()>);

    // Set closure as click handler for button
    window.document().unwrap().get_element_by_id("addTodoItem").unwrap().dyn_ref::<HtmlButtonElement>().unwrap()
        .set_onclick(Some(btn_handler.as_ref().unchecked_ref()));
    
    // We need to "forget" closure. Otherwise, Rust would clean it up. This leaks memory!
    // However, it is fine in our situation because we want a global event handler.
    btn_handler.forget();
}
// endregion: Manipulate DOM from Rust using web_sys
