use anyhow::{Result, anyhow};
use wasmtime::component::{Component, HasSelf, Linker};
use wasmtime::{Engine, Store};
mod state;

mod bindings {
    wasmtime::component::bindgen!({
        path: "./add.wit",
        world: "calculator"
    });
}
use bindings::docs::calculator::logger::Host;
use bindings::exports::docs::calculator::calculate::Op;
use state::States;
impl Host for States {
    fn info(&mut self, msg: String) {
        println!("Host Run:{}", msg)
    }
}

fn main() -> Result<()> {
    let engine = Engine::default();
    // Construct component
    let component =
        Component::from_file(&engine, "wasmc.wasm").map_err(|_| anyhow!("can't find component"))?;
    let wasi_view = States::new();
    let mut store = Store::new(&engine, wasi_view);
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)
        .map_err(|_| anyhow!("can't find component"))?;
    bindings::docs::calculator::logger::add_to_linker::<_, HasSelf<_>>(
        &mut linker,
        |state: &mut States| state,
    )
    .map_err(|_| anyhow!("can't find component"))?;
    let instance = bindings::Calculator::instantiate(&mut store, &component, &linker)
        .map_err(|_| anyhow!("instantiate error"))?;
    let result = instance
        .docs_calculator_calculate()
        .call_eval_expression(&mut store, Op::Add, 3, 4)
        .map_err(|_| anyhow!("compute error"))?;
    println!("result {}", result);
    // let instance = bindings::Calculator::instantiate(&mut store, &component, &linker)
    //     .map_err(|e| anyhow!("instantiate error"))?;
    // let instance = linker.instantiate(&component)?;
    // let processor =
    //     bindings::docs::calculator::calculator::Calculate::new(&instance)?;

    //     let result = processor.eval_expression(bindings::docs::calculator::calculator::Calculate::Op::Add, 3, 4)?;
    //     println!("Result: {}", result);
    // let instance = bindings::Adder::instantiate(&mut store, &component, &linker)
    //         .context("Failed to instantiate the example world")?;

    // Call the add function on instance

    Ok(())
}
