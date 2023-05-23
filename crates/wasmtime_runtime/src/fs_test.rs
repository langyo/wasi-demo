use anyhow::Result;
use std::path::Path;

use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, Dir};

pub fn fs_test() -> Result<()> {
    todo!("Not done");

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .preopened_dir(
            Dir::open_ambient_dir("/", wasmtime_wasi::ambient_authority()).unwrap(),
            Path::new("target"),
        )?
        .build();
    let mut store = Store::new(&engine, wasi);

    let module = Module::from_file(&engine, "target/wasm32-wasi/debug/wasi_fs_demo.wasm")?;
    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    Ok(())
}
