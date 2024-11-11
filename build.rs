use std::env;
use std::path::*;
use std::process::*;

fn compile_resource(path: PathBuf) {
    if Command::new("rc").status().is_ok() {
        if !path.exists() {
            println!("cargo:warning={} not found", path.display());
        } else {
            let res_file = path.file_stem().unwrap().to_str().unwrap().to_owned() + ".res";
            let res: PathBuf = [root().as_str(), "target", res_file.as_str()]
                .iter()
                .collect();

            Command::new("rc")
                .args(["/fo", res.to_str().unwrap(), path.to_str().unwrap()])
                .status()
                .unwrap();

            println!("cargo::rustc-link-arg-bins={}", res.to_str().unwrap());
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

    let rc = root.join("path").join("app.rc");
    compile_resource(rc);

    let manifest = root.join("data").join("app.manifest");
    embed_manifest(manifest);
}
