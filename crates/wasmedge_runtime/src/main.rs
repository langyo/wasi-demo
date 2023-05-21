use anyhow::Result;
use std::{io::Read, path::Path, process::Command};

use wasmedge_sdk::{
    error::HostFuncError, host_function, params, wat2wasm, Caller, Executor, ImportObjectBuilder,
    Module, Store, WasmValue,
};

fn main() -> Result<()> {
    // TODO - Not works

    if !Path::new("target/wasm32-wasi/debug/wasi_hello_world.wasm").exists() {
        let output = Command::new("cargo")
            .args(&[
                "build",
                "--package",
                "wasi_hello_world",
                "--target",
                "wasm32-wasi",
            ])
            .output()
            .expect("failed to execute process");
        println!("cargo build: {:?}", output);
    }
    let wasm_bytes = std::fs::read("target/wasm32-wasi/debug/wasi_hello_world.wasm")?;

    let module = Module::from_bytes(None, &wasm_bytes)?;
    let mut executor = Executor::new(None, None)?;
    let mut store = Store::new()?;

    let extern_instance = store.register_named_module(&mut executor, "extern", &module)?;
    let run = extern_instance
        .func("run")
        .ok_or_else(|| anyhow::Error::msg("Not found exported function named 'run'."))?;
    run.call(&mut executor, params!())?;

    Ok(())
}
