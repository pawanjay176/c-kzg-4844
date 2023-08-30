use std::env;
use std::path::PathBuf;

fn main() {
    let cargo_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let root_dir = cargo_dir
        .parent()
        .expect("rust dir is nested")
        .parent()
        .expect("bindings dir is nested");

    let blst_base_dir = root_dir.join("blst");
    // Obtain the header files of blst
    let blst_headers_dir = blst_base_dir.join("bindings");

    let c_src_dir = root_dir.join("src");

    let mut cc = cc::Build::new();

    #[cfg(windows)]
    cc.flag("-D_CRT_SECURE_NO_WARNINGS");

    cc.include(blst_headers_dir.clone());
    cc.warnings(false);
    cc.file(c_src_dir.join("c_kzg_4844.c"));

    cc.try_compile("ckzg").expect("Failed to compile ckzg");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_out_path = out_dir.join("generated.rs");
    let header_file_path = c_src_dir.join("c_kzg_4844.h");
    let header_file = header_file_path.to_str().expect("valid header file");

    make_bindings(
        header_file,
        &blst_headers_dir.to_string_lossy(),
        bindings_out_path,
    );

    // Finally, tell cargo this provides ckzg
    println!("cargo:rustc-link-lib=ckzg");
}

fn make_bindings<P>(header_path: &str, blst_headers_dir: &str, bindings_out_path: P)
where
    P: AsRef<std::path::Path>,
{
    use bindgen::Builder;

    let bindings = Builder::default()
        /*
         * Header definitions.
         */
        .header(header_path)
        .clang_args([format!("-I{blst_headers_dir}")])
        // Get bindings only for the header file.
        .allowlist_file(".*c_kzg_4844.h")
        /*
         * Cleanup instructions.
         */
        // Remove stdio definitions related to FILE.
        .opaque_type("FILE")
        // Remove the definition of FILE to use the libc one, which is more convenient.
        .blocklist_type("FILE")
        /*
         * Re-build instructions
         */
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap();

    bindings
        .write_to_file(bindings_out_path)
        .expect("Failed to write bindings");
}
