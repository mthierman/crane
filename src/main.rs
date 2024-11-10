// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(dead_code)]

use serde::Deserialize;
use std::fs::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Command;
use windows::{Win32::Foundation::HANDLE, Win32::UI::Shell::*};

#[derive(Deserialize, Debug)]
struct Manifest {
    packages: Vec<String>,
}

fn app_data() -> String {
    unsafe {
        SHGetKnownFolderPath(
            &FOLDERID_LocalAppData,
            KNOWN_FOLDER_FLAG::default(),
            HANDLE::default(),
        )
        .unwrap()
        .to_string()
        .unwrap()
    }
}

fn package_cache() -> PathBuf {
    [app_data().as_str(), "crane", "packages"].iter().collect()
}

fn main() {
    let package_cache = package_cache();

    if !package_cache.exists() {
        create_dir_all(&package_cache).unwrap();
    }

    let manifest_file_path = PathBuf::from("crane.json");
    let manifest_file = File::open(&manifest_file_path);

    match manifest_file {
        Ok(_) => {
            let reader = BufReader::new(manifest_file.unwrap());

            let u = serde_json::from_reader::<_, Manifest>(reader).unwrap();

            for package in u.packages.iter() {
                let split: Vec<&str> = package.split(":").collect();

                match split[0] {
                    "gh" => {
                        let repo = split[1].split("@").next().unwrap();
                        let branch = split[1].split("@").nth(1).unwrap();
                        println!("Installing {}@{}", repo, branch);

                        let output = Command::new("gh")
                            .current_dir(&package_cache)
                            .args(["repo", "clone", repo, "--", "--branch", branch, "--depth=1"])
                            .output()
                            .unwrap();
                        // let printout = String::from_utf8(output.stdout).unwrap();
                        // println!("{}", printout);
                    }
                    "nuget" => {
                        let package = split[1].split("@").next().unwrap();
                        let version = split[1].split("@").nth(1).unwrap();
                        println!("Installing {}@{}...", package, version);

                        let output = Command::new("nuget")
                            .current_dir(&package_cache)
                            .args(["install", package, "-Version", version])
                            .output()
                            .unwrap();
                        // let printout = String::from_utf8(output.stdout).unwrap();
                        // println!("{}", printout);
                    }
                    _ => {
                        println!("ERROR!")
                    }
                }
            }
        }
        Err(_) => {
            println!("Manifest doesn't exist, creating...");
            let _ = File::create(&manifest_file_path);
        }
    }
}
