fn main() {
    // Generate Rust bindings for libsystemd using bindgen. The `wrapper.h`
    // file only includes the desired systemd headers, which keeps the
    // resulting bindings focused on the APIs we need.
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/include/systemd")
        .generate()
        .expect("Unable to generate bindings for systemd");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=dylib=systemd");
    println!("cargo:rustc-link-search=native=/usr/lib64");
}
