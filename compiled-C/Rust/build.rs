use std::process::Command;
fn main() {
    Command::new("make").args(&["-C", "..", "-f", "Makefile-sofile"]).status().unwrap();

    println!("cargo:rustc-link-search=native=../");
    println!("cargo:rustc-link-lib=dylib=c_example");
}
