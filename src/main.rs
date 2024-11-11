#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use serde::Deserialize;
use std::fs::*;
use std::io::BufReader;
use std::os::windows::fs::symlink_dir;
use std::path::PathBuf;
use std::process::Command;
use windows::{Win32::Foundation::HANDLE, Win32::UI::Shell::*};

#[derive(Deserialize, Debug)]
struct Manifest {
    packages: Vec<String>,
}

struct GitHub {
    owner: String,
    repo: String,
    branch: String,
}

struct Nuget {
    package: String,
    version: String,
}

struct Crane {
    cache: PathBuf,
    manifest: PathBuf,
    reader: BufReader<File>,
}

impl Crane {
    fn new() -> Self {
        let manifest = PathBuf::from("crane.json");
        let manifest_file = File::open(&manifest).unwrap();

        Self {
            cache: Crane::cache(),
            manifest: manifest,
            reader: BufReader::new(manifest_file),
        }
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

    fn cache() -> PathBuf {
        [Crane::app_data().as_str(), "crane", "packages"]
            .iter()
            .collect()
    }
}

fn main() {
    let crane = Crane::new();

    if !crane.cache.exists() {
        create_dir_all(&crane.cache).unwrap();
    }

    match crane.reader.get_ref().metadata() {
        Ok(_) => {
            let manifest = serde_json::from_reader::<_, Manifest>(crane.reader).unwrap();

            for package in manifest.packages.iter() {
                let provider = package.split(":").nth(0).unwrap();
                println!("{}", provider);

                match provider {
                    "gh" => {
                        let owner = package
                            .split(":")
                            .nth(1)
                            .unwrap()
                            .split("/")
                            .nth(0)
                            .unwrap();
                        println!("{}", &owner);

                        let repo = package
                            .split(":")
                            .nth(1)
                            .unwrap()
                            .split("@")
                            .nth(0)
                            .unwrap()
                            .split("/")
                            .nth(1)
                            .unwrap();
                        println!("{}", &repo);

                        let branch = package
                            .split(":")
                            .nth(1)
                            .unwrap()
                            .split("@")
                            .nth(1)
                            .unwrap();
                        println!("{}", &branch);

                        let mut current_dir = crane.cache.clone();
                        current_dir.push(owner);

                        if !current_dir.exists() {
                            create_dir_all(&current_dir).unwrap();
                        }

                        let repo_to_clone: String = owner.to_owned() + "/" + repo;
                        let out_dir = repo.to_owned() + "/" + branch;

                        Command::new("gh")
                            .current_dir(&current_dir)
                            .args([
                                "repo",
                                "clone",
                                &repo_to_clone,
                                &out_dir,
                                "--",
                                "--branch",
                                &branch,
                                "--depth=1",
                            ])
                            .output()
                            .unwrap();

                        current_dir.push(repo);
                        current_dir.push(branch);

                        println!("{}", current_dir.display());
                        let mut link = std::env::current_dir().unwrap();
                        link.push("crane_packages");
                        if !link.exists() {
                            create_dir_all(&link).unwrap();
                        }
                        link.push(repo);

                        symlink_dir(current_dir, link).unwrap();
                    }
                    "nuget" => {
                        // let package = split[1].split("@").next().unwrap();
                        // let version = split[1].split("@").nth(1).unwrap();
                        // println!("Installing {}@{}...", package, version);

                        // Command::new("nuget")
                        //     .current_dir(&package_cache)
                        //     .args(["install", package, "-Version", version])
                        //     .output()
                        //     .unwrap();
                    }
                    _ => {
                        println!("ERROR!")
                    }
                }
            }
        }
        Err(_) => {
            println!("Manifest doesn't exist, creating...");
            let _ = File::create(&crane.manifest);
        }
    }
}
