use std::env;
use std::path::*;
use std::process::*;

fn root() -> String {
    let root = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    root
}

fn compile_resource(path: PathBuf) {
    let rc = Command::new("rc").status();
    if rc.is_ok() {
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

    let rc: PathBuf = [root().as_str(), "data", "app.rc"].iter().collect();
    compile_resource(rc);

    let manifest: PathBuf = [root().as_str(), "data", "app.manifest"].iter().collect();
    embed_manifest(manifest);
}
