use common::*;
use std::env;
use std::path::*;
use std::process::*;

fn compile_resource(rc_file: PathBuf) {
    let rc = resource_compiler();

    if Command::new(&rc).status().is_ok() {
        if !rc_file.exists() {
            println!("cargo:warning={} not found", rc_file.display());
        } else {
            let root = env::current_dir().unwrap();
            let res_file = root.join("target").join(format!(
                "{}.res",
                rc_file.file_stem().unwrap().to_str().unwrap()
            ));

            Command::new(&rc)
                .args(["/fo", res_file.to_str().unwrap(), rc_file.to_str().unwrap()])
                .status()
                .unwrap();

            println!("cargo::rustc-link-arg-bins={}", res_file.to_str().unwrap());
        }
    } else {
        println!("cargo:warning=rc.exe not found");
    }
}

fn embed_manifest(path: PathBuf) {
    if !path.exists() {
        println!("cargo:warning={} not found", path.display());
    } else {
        println!("cargo::rustc-link-arg-bins=/MANIFEST:EMBED");
        println!(
            "cargo::rustc-link-arg-bins=/MANIFESTINPUT:{}",
            path.to_str().unwrap()
        );
    }
}

fn main() {
    println!("cargo::rustc-link-arg-bins=/WX");
    // println!("cargo::rustc-link-arg-bins=/LINKREPROFULLPATHRSP:crane.rsp");

    let root = env::current_dir().unwrap();

    let rc = root.join("data").join("app.rc");
    compile_resource(rc);

    let manifest = root.join("data").join("app.manifest");
    embed_manifest(manifest);
}
