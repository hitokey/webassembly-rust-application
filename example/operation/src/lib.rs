
extern crate serde_json;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub unsafe fn add_two_ints(a: i32, b: i32) -> i32 {
    a + b
}