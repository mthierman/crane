use std::{path::PathBuf, process::Command};
use windows::Win32::{Foundation::HANDLE, UI::Shell::*};

pub fn vswhere() -> PathBuf {
    PathBuf::from(unsafe {
        SHGetKnownFolderPath(
            &FOLDERID_ProgramFilesX86,
            KF_FLAG_DONT_VERIFY,
            HANDLE::default(),
        )
        .unwrap()
        .to_string()
        .unwrap()
    })
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
        r#"cmd /K 'call "{}" > NUL && pwsh -C "$env:WindowsSdkVerBinPath"'"#,
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
