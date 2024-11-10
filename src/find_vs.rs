// use std::env;
// use std::path::*;
// use std::process::Command;

// fn find_vs() {
//     let program_files = env::var("ProgramFiles(x86)").unwrap();

//     let vswhere: PathBuf = [
//         program_files.as_str(),
//         "Microsoft Visual Studio",
//         "Installer",
//         "vswhere.exe",
//     ]
//     .iter()
//     .collect();

//     // println!(
//     //     "cargo:warning={}",
//     //     vswhere.into_os_string().into_string().unwrap()
//     // );

//     let output = Command::new(vswhere)
//         .args(["-products", "*", "-latest", "-property", "installationPath"])
//         .output()
//         .unwrap();

//     // println!(
//     //     "cargo:warning={}",
//     //     String::from_utf8(output.stdout).unwrap()
//     // );

//     let installation_path: PathBuf = [
//         String::from_utf8(output.stdout).unwrap().as_str(),
//         "Common7",
//         "Tools",
//         "Launch-VsDevShell.ps1",
//     ]
//     .iter()
//     .collect();

//     Command::new("pwsh")
//         .args([
//             // "&",
//             installation_path
//                 .into_os_string()
//                 .into_string()
//                 .unwrap()
//                 .as_str(),
//             "-Arch",
//             "amd64",
//             "-HostArch",
//             "amd64",
//             "-SkipAutomaticLocation",
//         ])
//         .status()
//         .unwrap();
// }
