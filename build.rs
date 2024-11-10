use std::env;
use std::path::*;
use std::process::Command;

fn root() -> String {
    let root = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    root
}

fn embed_manifest(path: PathBuf) {
    if !path.exists() {
        println!("cargo:warning={}", "Manifest not found")
    } else {
        println!("cargo::rustc-link-arg-bins=/MANIFEST:EMBED");
        println!(
            "cargo::rustc-link-arg-bins=/MANIFESTINPUT:{}",
            path.to_str().unwrap()
        );
    }
}

fn compile_resource(path: PathBuf) {
    if !path.exists() {
        println!("cargo:warning={}", "RC not found")
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
}

fn linker_options(flags: &str) {
    println!("cargo::rustc-link-arg-bins={}", flags);
}

fn main() {
    let manifest: PathBuf = [root().as_str(), "data", "app.manifest"].iter().collect();
    embed_manifest(manifest);

    let rc: PathBuf = [root().as_str(), "data", "app.rc"].iter().collect();
    compile_resource(rc);

    linker_options("/WX");
}
