use std::env;

fn main() {
    // Generate the C header from the Rust source on every build, so the header
    // used by C code can never drift from the actual Rust API. Settings are
    // read from cbindgen.toml.
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::generate(crate_dir)
        .expect("unable to generate C bindings")
        .write_to_file("target/include/area_calculator_rs.h");
}
