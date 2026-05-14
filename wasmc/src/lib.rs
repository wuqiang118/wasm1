wit_bindgen::generate!({
    world: "calculator",
    path: "./wit",
});

use crate::exports::docs::calculator::calculate::{
    Guest, Op
};

use crate::docs::calculator::logger::{
    info
};

// 实现 WIT 接口
struct Calculator;

impl Guest for Calculator {
    fn eval_expression(op: Op, x: u32, y: u32) -> u32 {
        info("print ok");
        match op {
            Op::Add => x + y,
            Op::Subtract => x.saturating_sub(y),
        }
    }
}

// 导出组件
export!(Calculator);
