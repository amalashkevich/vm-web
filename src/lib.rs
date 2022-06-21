mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

mod vm_ta;
use crate::vm_ta::run_machine;


#[wasm_bindgen]
pub fn submit_code(byte_code: &str) {
    let result = run_machine(byte_code);
    println!("Result={:?}", result);
    alert(&format!("Result is {}", result));
}
