use anyhow::Result;
use std::{io::Read, path::Path, process::Command};

use wasmer::{Module, Store};
use wasmer_wasix::{Pipe, WasiEnv};

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
    let wasm_bytes = std::fs::read("target/wasm32-wasi/debug/wasi_hello_world.wasm")?;

    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let (stdout_tx, mut stdout_rx) = Pipe::channel();

    WasiEnv::builder("hello-world")
        .stdout(Box::new(stdout_tx))
        .run_with_store(module, &mut store)?;

    let mut buf = String::new();
    stdout_rx.read_to_string(&mut buf).unwrap();
    println!("{}", buf);

    Ok(())
}
