#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;
use battleship_wasm::SquareContentJS;
use battleship_game_logic::SquareContent;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn square_content() {
    assert_eq!(SquareContentJS::Water as u8, SquareContent::Water as u8);
    assert_eq!(SquareContentJS::HitShip as u8, SquareContent::HitShip as u8);
    assert_eq!(SquareContentJS::SunkenShip as u8, SquareContent::SunkenShip as u8);
}
