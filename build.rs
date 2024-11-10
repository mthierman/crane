use std::env;
use std::process::Command;

fn embed_manifest(path: &str) {
    let mut manifest = env::current_dir().unwrap();

    manifest.push(path);

    println!("cargo::rustc-link-arg-bins=/MANIFEST:EMBED");
    println!(
        "cargo::rustc-link-arg-bins=/MANIFESTINPUT:{}",
        manifest.to_str().unwrap()
    );
}

fn compile_resource(path: &str) {
    let current_dir = env::current_dir().unwrap();

    let mut rc = current_dir.clone();
    rc.push(path);

    let mut filename = String::from(rc.file_stem().unwrap().to_str().unwrap());
    filename.push_str(".res");

    let mut res = current_dir.clone();
    res.push(format!("target/{}", filename));

    Command::new("rc")
        .args(["/fo", "target/app.res", rc.to_str().unwrap()])
        .status()
        .unwrap();

    println!("cargo::rustc-link-arg-bins={}", res.to_str().unwrap());
}

fn linker_options(flags: &str) {
    println!("cargo::rustc-link-arg-bins={}", flags);
}

fn main() {
    embed_manifest("data/app.manifest");
    compile_resource("data/app.rc");
    linker_options("/WX");
}
