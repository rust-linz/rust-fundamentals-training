use std::process::Command;

fn main() {
    Command::new("make")
        .current_dir("../area_calculator_c")
        .status()
        .expect("failed to execute process");

    println!(r"cargo:rustc-link-search=all=../area_calculator_c");
    println!("cargo:rustc-link-lib=areacalc");
}
