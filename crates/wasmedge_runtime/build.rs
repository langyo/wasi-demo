pub fn main() {
    // TODO - Download the latest release from the github source

    let root_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    let include_dir = root_dir.join("lib").join("include").join("wasmedge");
    let lib_dir = root_dir.join("lib").join("lib");

    println!(
        "cargo:rustc-env=WASMEDGE_INCLUDE_DIR={}",
        include_dir.as_os_str().to_str().unwrap()
    );
    println!(
        "cargo:rustc-env=WASMEDGE_LIB_DIR={}",
        lib_dir.as_os_str().to_str().unwrap()
    );
}
