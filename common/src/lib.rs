use std::env;
use std::{path::PathBuf, process::Command};
use windows::core::GUID;
use windows::Win32::{Foundation::HANDLE, UI::Shell::*};

pub fn known_folder(rfid: &GUID) -> PathBuf {
    PathBuf::from(unsafe {
        SHGetKnownFolderPath(rfid, KF_FLAG_DONT_VERIFY, HANDLE::default())
            .unwrap()
            .to_string()
            .unwrap()
    })
}

pub fn vswhere() -> PathBuf {
    PathBuf::from(known_folder(&FOLDERID_ProgramFilesX86))
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

pub fn windows_kit() -> PathBuf {
    let script = format!(
        r#"cmd /C 'call "{}" > NUL && pwsh -C "$env:WindowsSdkVerBinPath"'"#,
        winsdk_bat().to_str().unwrap()
    );

    let output = Command::new("pwsh")
        .envs([
            ("VSCMD_ARG_HOST_ARCH", "x64"),
            ("VSCMD_ARG_TGT_ARCH", "x64"),
        ])
        .args(["-C", &script])
        .output()
        .unwrap();

    PathBuf::from(String::from_utf8(output.stdout).unwrap().trim())
}

pub fn resource_compiler() -> PathBuf {
    windows_kit().join("x64").join("rc.exe")
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
