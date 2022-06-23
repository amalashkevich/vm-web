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
    alert(&format!("Result is {}", result.unwrap()));
}

#[cfg(test)]
mod tests {
    use crate::vm_ta::run_machine;
    use std::thread;
    use std::time::Duration;

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
        assert_eq!(result.unwrap(), "4");
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
        assert_eq!(result.unwrap(), "192");
    }

    #[test]
    fn test_loop() {
        // x = 0
        // while x < 3 {
        //   print x
        //   x = x + 1
        // }
        let byte_code = "
            LOAD_VAL 0
            WRITE_VAR 'x'

            LABEL :loop1start
            READ_VAR 'x'
            LOAD_VAL 3
            CMP_LT
            POP_JUMP_IF_FALSE :loop1end

            READ_VAR 'x'
            PRINT 'x'

            READ_VAR 'x'
            LOAD_VAL 1
            ADD
            WRITE_VAR 'x'

            JUMP :loop1start

            LABEL :loop1end
            READ_VAR 'x'
            RETURN_VALUE
        ";
        let result = run_machine(byte_code);
        assert_eq!(result.unwrap(), "3");
    }

    #[test]
    fn test_send_channel() {
        let byte_code = "
            PUSH 'val'
            PUSH 'channel_name'
            SEND_CHANNEL
            ";

        run_machine(byte_code);
    }

    #[test]
    fn test_recv_channel() {
        let byte_code = "
            PUSH 'channel_name'
            RECV_CHANNEL
            RETURN_VALUE
            ";

        let result = run_machine(byte_code);
        assert_eq!(result.unwrap(), "recv_channel_value");
    }

    #[test]
    fn test_spawn() {
        let byte_code = "
            PUSH 'channel_recv'
            PUSH 'channel_send'
            SPAWN
            ";

        run_machine(byte_code);
        thread::sleep(Duration::from_millis(15));
    }
}