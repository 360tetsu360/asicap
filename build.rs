use std::env;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/lib/armv7", project_dir);
}