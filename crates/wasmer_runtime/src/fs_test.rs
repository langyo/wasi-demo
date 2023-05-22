use anyhow::Result;
use std::{path::Path, sync::Arc};

use wasmer::{Module, Store};
use wasmer_vfs::mem_fs::FileSystem as MemoryFileSystem;
use wasmer_vfs::{FileSystem, FsError, Metadata, OpenOptions, ReadDir};
use wasmer_wasix::{Pipe, WasiEnv};

#[derive(Debug, Clone)]
pub struct MemFS {
    inner: Arc<MemoryFileSystem>,
}

impl MemFS {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(MemoryFileSystem::default()),
        }
    }
}

impl FileSystem for MemFS {
    fn read_dir(&self, path: &Path) -> Result<ReadDir, FsError> {
        self.inner.read_dir(path)
    }
    fn create_dir(&self, path: &Path) -> Result<(), FsError> {
        self.inner.create_dir(path)
    }
    fn remove_dir(&self, path: &Path) -> Result<(), FsError> {
        self.inner.remove_dir(path)
    }
    fn rename(&self, from: &Path, to: &Path) -> Result<(), FsError> {
        self.inner.rename(from, to)
    }
    fn metadata(&self, path: &Path) -> Result<Metadata, FsError> {
        self.inner.metadata(path)
    }
    fn symlink_metadata(&self, path: &Path) -> Result<Metadata, FsError> {
        self.inner.symlink_metadata(path)
    }
    fn remove_file(&self, path: &Path) -> Result<(), FsError> {
        self.inner.remove_file(path)
    }

    fn new_open_options(&self) -> OpenOptions {
        self.inner.new_open_options()
    }
}

pub async fn fs_test() -> Result<()> {
    todo!("Not completed");
    // https://github.com/wasmerio/wasmer-js

    let wasm_bytes = std::fs::read("target/wasm32-wasi/debug/wasi_fs_demo.wasm")?;

    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let (stdout_tx, _) = Pipe::channel();

    let vfs = MemFS::new();
    let mut builder = WasiEnv::builder("fs-demo")
        .stdout(Box::new(stdout_tx))
        .preopen_dir("/")?;
    builder.set_fs(Box::new(vfs));
    builder.run_with_store(module, &mut store)?;

    let dir = vfs.read_dir(Path::new("/"))?;
    println!("Virtual FS dir: {:?}", dir);
    let mut file = vfs
        .new_open_options()
        .read(true)
        .open(Path::new("/test.txt"))?;
    let mut buf = Vec::<u8>::new();
    file.read_to_end(&mut buf)?;
    println!("Virtual FS file: {:?}", buf);

    Ok(())
}
