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

fn find_vs() {
    let program_files = env::var("ProgramFiles(x86)").unwrap();

    let vswhere: PathBuf = [
        program_files.as_str(),
        "Microsoft Visual Studio",
        "Installer",
        "vswhere.exe",
    ]
    .iter()
    .collect();

    // println!(
    //     "cargo:warning={}",
    //     vswhere.into_os_string().into_string().unwrap()
    // );

    let output = Command::new(vswhere)
        .args(["-products", "*", "-latest", "-property", "installationPath"])
        .output()
        .unwrap();

    // println!(
    //     "cargo:warning={}",
    //     String::from_utf8(output.stdout).unwrap()
    // );

    let installation_path: PathBuf = [
        String::from_utf8(output.stdout).unwrap().as_str(),
        "Common7",
        "Tools",
        "Launch-VsDevShell.ps1",
        "-Arch",
        "amd64",
        "-HostArch",
        "amd64",
        "-SkipAutomaticLocation",
    ]
    .iter()
    .collect();

    Command::new("pwsh")
        .args([
            "&",
            installation_path
                .into_os_string()
                .into_string()
                .unwrap()
                .as_str(),
        ])
        .status()
        .unwrap();
}

fn embed_manifest(path: PathBuf) {
    if !path.exists() {
        println!("cargo:warning={}", "Manifest not found");
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
        println!("cargo:warning={}", "RC not found");
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
    find_vs();

    let manifest: PathBuf = [root().as_str(), "data", "app.manifest"].iter().collect();
    embed_manifest(manifest);

    let rc: PathBuf = [root().as_str(), "data", "app.rc"].iter().collect();
    compile_resource(rc);

    linker_options("/WX");
}
