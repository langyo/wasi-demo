use anyhow::Result;
use std::process::Command;

use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

mod fs_test;

fn hello_world() -> Result<()> {
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

fn main() -> Result<()> {
    Command::new("cargo")
        .args(&[
            "build",
            "--package",
            "wasi_hello_world",
            "--target",
            "wasm32-wasi",
        ])
        .spawn()?
        .wait_with_output()?;

    Command::new("cargo")
        .args(&[
            "build",
            "--package",
            "wasi_fs_demo",
            "--target",
            "wasm32-wasi",
        ])
        .spawn()?
        .wait_with_output()?;

    hello_world()?;

    fs_test::fs_test()?;

    Ok(())
}
