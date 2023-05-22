use anyhow::Result;
use std::{io::Read, process::Command};

use wasmer::{Module, Store};
use wasmer_wasix::{Pipe, WasiEnv};

mod fs_test;

fn hello_world() -> Result<()> {
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

// #[tokio::main]
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

    // fs_test::fs_test().await?;

    Ok(())
}
