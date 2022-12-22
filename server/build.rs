use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_dir = Path::new(&project_dir).parent().unwrap();
    let build_type = env::var("PROFILE").unwrap();
    workspace_dir.join("target").join(build_type)
}

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let raspberrypi_model = env::var("RASPBERRYPI_VER").ok();

    println!("cargo:rerun-if-changed={}.cargo/config.toml", project_dir);

    if cfg!(target_os = "linux") {
        if let Some(model) = raspberrypi_model {
            let arch = match &*model {
                "raspi1" => "armv6",
                "raspi2" | "raspi3" | "raspi4" => "armv7",
                _ => panic!(),
            };
            println!(
                "cargo:rustc-link-search=native={}/lib/raspberrypi/{}",
                project_dir, arch
            );
        } else {
            println!(
                "cargo:rustc-link-search=native={}/lib/{}/{}",
                project_dir, target_os, target_arch
            );
        }
        println!("cargo:rustc-link-lib=static=ASICamera2");
        println!("cargo:rustc-link-lib=static=usb-1.0");
        println!("cargo:rustc-link-lib=dylib=udev");
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if cfg!(target_os = "macos") {
        println!(
            "cargo:rustc-link-search=native={}/lib/{}",
            project_dir, target_os
        );
        println!("cargo:rustc-link-lib=static=ASICamera2");
        println!("cargo:rustc-link-lib=static=usb-1.0");
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if cfg!(target_os = "windows") {
        println!(
            "cargo:rustc-link-search=native={}/lib/{}/{}",
            project_dir, target_os, target_arch
        );
        println!("cargo:rustc-link-lib=static=ASICamera2");
        let target_dir = get_output_path();
        let src = Path::join(
            &env::current_dir().unwrap(),
            format!("lib\\windows\\{}\\ASICamera2.dll", target_arch),
        );
        let dest = Path::join(Path::new(&target_dir), Path::new("ASICamera2.dll"));
        fs::copy(src, dest).unwrap();
    }
}
