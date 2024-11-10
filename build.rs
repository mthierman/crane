use std::env;
use std::path::*;
use std::process::Command;

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
    let current_dir = env::current_dir().unwrap();

    let mut filename = String::from(path.file_stem().unwrap().to_str().unwrap());
    filename.push_str(".res");

    let mut res = current_dir.clone();
    res.push(format!("target/{}", filename));

    if !path.exists() {
        println!("cargo:warning={}", "RC not found")
    } else {
        Command::new("rc")
            .args(["/fo", "target/app.res", path.to_str().unwrap()])
            .status()
            .unwrap();

        println!("cargo::rustc-link-arg-bins={}", res.to_str().unwrap());
    }
}

// fn linker_options(flags: &str) {
//     println!("cargo::rustc-link-arg-bins={}", flags);
// }

fn main() {
    let root = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let manifest: PathBuf = [root.as_str(), "data", "app.manifest"].iter().collect();
    embed_manifest(manifest);

    let rc: PathBuf = [root.as_str(), "data", "app.rc"].iter().collect();
    compile_resource(rc);

    // linker_optins("/WX");
}
