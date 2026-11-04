//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[allow(clippy::eq_op)]
#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}
