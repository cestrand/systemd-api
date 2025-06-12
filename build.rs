fn main() {
    // Generate Rust bindings for libsystemd using bindgen. The `wrapper.h`
    // file only includes the desired systemd headers, which keeps the
    // resulting bindings focused on the APIs we need.
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/include")
        .generate()
        .expect("Unable to generate bindings for systemd");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=dylib=systemd");
    println!("cargo:rustc-link-search=native=/usr/lib64");
}