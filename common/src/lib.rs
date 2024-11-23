use std::env;
use std::{path::PathBuf, process::Command};
use windows::core::GUID;
use windows::Win32::{Foundation::HANDLE, UI::Shell::*};

pub fn known_folder(id: &GUID, flag: KNOWN_FOLDER_FLAG) -> PathBuf {
    PathBuf::from(unsafe {
        SHGetKnownFolderPath(id, flag, HANDLE::default())
            .unwrap()
            .to_string()
            .unwrap()
    })
}

pub fn root() -> PathBuf {
    env::current_dir().unwrap()
}

pub fn vswhere() -> PathBuf {
    PathBuf::from(known_folder(&FOLDERID_ProgramFilesX86, KF_FLAG_DONT_VERIFY))
        .join("Microsoft Visual Studio")
        .join("Installer")
        .join("vswhere.exe")
}

pub fn install_path() -> PathBuf {
    PathBuf::from(
        String::from_utf8(
            Command::new(vswhere())
                .args(["-property", "resolvedInstallationPath"])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .trim(),
    )
}

pub fn winsdk_bat() -> PathBuf {
    install_path()
        .join("Common7")
        .join("Tools")
        .join("vsdevcmd")
        .join("core")
        .join("winsdk.bat")
}

pub fn windows_kit(arch: &str) -> PathBuf {
    let output = Command::new("cmd")
        .envs([("VSCMD_ARG_HOST_ARCH", arch), ("VSCMD_ARG_TGT_ARCH", arch)])
        .args([
            "/v:on",
            "/C",
            winsdk_bat().to_str().unwrap(),
            ">",
            "NUL",
            "&",
            "echo",
            "!WindowsSdkVerBinPath!",
        ])
        .output()
        .unwrap();

    PathBuf::from(String::from_utf8(output.stdout).unwrap().trim())
}

pub fn resource_compiler() -> PathBuf {
    windows_kit("x64").join("x64").join("rc.exe")
}

pub fn compile_resource(rc_file: PathBuf) {
    let rc = resource_compiler();

    if rc_file.exists() {
        let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

        let res_file = root.join("target").join(format!(
            "{}.res",
            rc_file.file_stem().unwrap().to_str().unwrap()
        ));

        Command::new(&rc)
            .args(["/fo", res_file.to_str().unwrap(), rc_file.to_str().unwrap()])
            .status()
            .unwrap();

        println!("cargo::rustc-link-arg-bins={}", res_file.to_str().unwrap());
    } else {
        println!("cargo:warning={} not found", rc_file.display());
    }
}

pub fn embed_manifest(path: PathBuf) {
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
