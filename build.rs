use std::env;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/lib/armv7", project_dir);
    println!("cargo:rustc-link-lib=static=ASICamera2");
    println!("cargo:rustc-link-lib=static=usb-1.0");
    println!("cargo:rustc-link-lib=dylib=udev");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
