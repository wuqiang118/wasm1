use wasm_bindgen::prelude::*;

/*
 * 声明JS 的 import
 */
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace= window)]
    fn js_on_add_result(value: i32);
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    let res = a + b;
    log(&format!("WASM : {} + {}={}", a, b, res));
    js_on_add_result(res);
    res
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    let msg = format!("Hello, {} from Rust WASM!", name);
    log(&msg);
    msg
}
