use common::*;
use std::env;

fn main() {
    println!("cargo::rustc-link-arg-bins=/WX");
    // println!("cargo::rustc-link-arg-bins=/LINKREPROFULLPATHRSP:crane.rsp");

    let root = env::current_dir().unwrap();

    let rc = root.join("data").join("app.rc");
    compile_resource(rc);

    let manifest = root.join("data").join("app.manifest");
    embed_manifest(manifest);
}
