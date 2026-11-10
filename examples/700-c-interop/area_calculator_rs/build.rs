use std::{env, path::PathBuf, process::Command};

fn main() {
    let c_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../area_calculator_c");

    // The C sources live outside this package, so Cargo would not notice when
    // they change. Tell it explicitly which files should trigger a rebuild.
    for file in ["area_calculator.c", "area_calculator.h", "Makefile"] {
        println!("cargo:rerun-if-changed={}", c_dir.join(file).display());
    }

    // `status()` only fails if `make` cannot be started. A failing compile
    // shows up in the exit status, so we have to check it ourselves.
    let status = Command::new("make")
        .current_dir(&c_dir)
        .status()
        .expect("failed to execute make");
    assert!(status.success(), "make failed with {status}");

    println!("cargo:rustc-link-search=native={}", c_dir.display());
    println!("cargo:rustc-link-lib=static=areacalc");
}
