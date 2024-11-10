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

    let manifest_file = File::open(&PathBuf::from("crane.json"));

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
                        println!("{} - {}", repo, branch);

                        // let output_directory: PathBuf = ["crane_packages", repo].iter().collect();

                        // if !output_directory.exists() {
                        //     let _ = create_dir_all(&output_directory);
                        // }

                        let output = Command::new("gh")
                            .current_dir(&cache)
                            .args(["repo", "clone", repo, "--", "--branch", branch, "--depth=1"])
                            .output()
                            .unwrap();
                        let printout = String::from_utf8(output.stdout).unwrap();
                        println!("{}", printout);
                    }
                    "nuget" => {
                        let package = split[1].split("@").next().unwrap();
                        let version = split[1].split("@").nth(1).unwrap();
                        println!("{} - {}", package, version);

                        let output = Command::new("nuget")
                            .current_dir(&cache)
                            .args(["install", package, "-Version", version])
                            .output()
                            .unwrap();
                        let printout = String::from_utf8(output.stdout).unwrap();
                        println!("{}", printout);
                    }
                    _ => {
                        println!("ERROR!")
                    }
                }
            }
        }
        Err(_) => {
            println!("crane.json doesn't exist...");
            let _ = File::create(&path);
        }
    }
}
