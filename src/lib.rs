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

#[cfg(test)]
mod tests {
    use crate::vm_ta::run_machine;

    #[test]
    fn test_bytecode1() {
        let byte_code = "
            LOAD_VAL 1
            WRITE_VAR 'x'

            LOAD_VAL 2
            WRITE_VAR 'y'

            READ_VAR 'x'
            LOAD_VAL 1
            ADD

            READ_VAR 'y'
            MULTIPLY

            RETURN_VALUE
        ";

    let result = run_machine(byte_code);
    assert_eq!(result, "4");
    }

    #[test]
    fn test_bytecode2() {
        let byte_code = "
            LOAD_VAL 2
            WRITE_VAR 'x'

            LOAD_VAL 8
            WRITE_VAR 'y'

            READ_VAR 'x'
            LOAD_VAL 1
            ADD

            READ_VAR 'y'
            MULTIPLY

            READ_VAR 'y'
            MULTIPLY

            RETURN_VALUE
            ";

    let result = run_machine(byte_code);
    assert_eq!(result, "192");
    }

}
