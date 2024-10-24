
extern crate wasm_bindgen;
extern crate serde_json;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub unsafe fn add(a: i32, b: i32) -> i32 {
    a + b
}