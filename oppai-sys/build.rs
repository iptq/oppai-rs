use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .define("OPPAI_IMPLEMENTATION", None)
        .files(&["oppai/oppai.c", "oppai/main.c"])
        .compile("oppai");
    println!("cargo:rustc-link-lib=oppai");
}
