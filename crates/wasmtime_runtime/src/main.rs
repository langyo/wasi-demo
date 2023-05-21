use anyhow::Result;
use std::{path::Path, process::Command};

use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

fn main() -> Result<()> {
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

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    let module = Module::from_file(&engine, "target/wasm32-wasi/debug/wasi_hello_world.wasm")?;
    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    Ok(())
}
