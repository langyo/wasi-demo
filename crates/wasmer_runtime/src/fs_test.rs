use anyhow::Result;
use std::path::Path;

use virtual_fs::{AsyncReadExt, FileSystem};
use wasmer::{Module, Store};
use wasmer_wasix::{Pipe, WasiEnv};

pub async fn fs_test() -> Result<()> {
    let wasm_bytes = std::fs::read("target/wasm32-wasi/debug/wasi_fs_demo.wasm")?;

    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let (stdout_tx, _) = Pipe::channel();

    let vfs = Box::new(virtual_fs::mem_fs::FileSystem::default());
    let mut builder = WasiEnv::builder("fs-demo")
        .stdout(Box::new(stdout_tx))
        .preopen_dir("/")?;
    builder.set_fs(vfs.clone());
    builder.run_with_store(module, &mut store)?;

    let dir = vfs.read_dir(Path::new("/"))?;
    println!("Virtual FS dir: {:?}", dir);
    let mut file = vfs
        .new_open_options()
        .read(true)
        .open(Path::new("/test.txt"))?;
    let mut buf = Vec::<u8>::new();
    file.read_to_end(&mut buf).await?;
    println!("Virtual FS file: {:?}", buf);

    Ok(())
}
