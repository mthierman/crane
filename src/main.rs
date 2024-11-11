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
    root: PathBuf,
    packages: PathBuf,
    manifest: PathBuf,
    reader: BufReader<File>,
    links: PathBuf,
}

impl Crane {
    fn new() -> Self {
        let manifest = PathBuf::from("crane.json");
        let manifest_file = File::open(&manifest).unwrap();
        let mut links = std::env::current_dir().unwrap();
        links.push("crane_packages");

        Self {
            root: Crane::root(),
            packages: Crane::cache(),
            manifest: manifest,
            reader: BufReader::new(manifest_file),
            links: links,
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

    fn root() -> PathBuf {
        [Crane::app_data().as_str(), "crane"].iter().collect()
    }

    fn cache() -> PathBuf {
        Crane::root().join("packages")
    }
}

fn main() {
    let crane = Crane::new();

    if !crane.root.exists() {
        create_dir_all(&crane.root).unwrap();
    }

    if !crane.packages.exists() {
        create_dir_all(&crane.packages).unwrap();
    }

    if !crane.links.exists() {
        create_dir_all(&crane.links).unwrap();
    }

    match crane.reader.get_ref().metadata() {
        Ok(_) => {
            let manifest = serde_json::from_reader::<_, Manifest>(crane.reader).unwrap();

            for package in manifest.packages.iter() {
                let provider = package.split(":").nth(0).unwrap();

                match provider {
                    "gh" => {
                        // let owner = package
                        //     .split(":")
                        //     .nth(1)
                        //     .unwrap()
                        //     .split("/")
                        //     .nth(0)
                        //     .unwrap();
                        // println!("{}", &owner);

                        // let repo = package
                        //     .split(":")
                        //     .nth(1)
                        //     .unwrap()
                        //     .split("@")
                        //     .nth(0)
                        //     .unwrap()
                        //     .split("/")
                        //     .nth(1)
                        //     .unwrap();
                        // println!("{}", &repo);

                        // let branch = package
                        //     .split(":")
                        //     .nth(1)
                        //     .unwrap()
                        //     .split("@")
                        //     .nth(1)
                        //     .unwrap();
                        // println!("{}", &branch);

                        // let mut out_dir = crane.packages.clone();
                        // out_dir.push("gh");
                        // out_dir.push(owner);
                        // println!("{}", out_dir.display());

                        // if !out_dir.exists() {
                        //     create_dir_all(&out_dir).unwrap();
                        // }

                        // Command::new("gh")
                        //     .current_dir(&out_dir)
                        //     .args([
                        //         "repo",
                        //         "clone",
                        //         String::from(owner.to_owned() + "/" + repo).as_str(),
                        //         String::from(repo.to_owned() + "/" + branch).as_str(),
                        //         "--",
                        //         "--branch",
                        //         &branch,
                        //         "--depth=1",
                        //     ])
                        //     .output()
                        //     .unwrap();

                        // out_dir.push(repo);
                        // out_dir.push(branch);
                        // println!("{}", out_dir.display());

                        // let mut link = crane.links.clone();
                        // link.push(repo);

                        // if !link.exists() {
                        //     symlink_dir(&out_dir, link).unwrap();
                        // }
                    }
                    "nuget" => {
                        let package_name = package
                            .split(":")
                            .nth(1)
                            .unwrap()
                            .split("@")
                            .nth(0)
                            .unwrap();

                        let version = package
                            .split(":")
                            .nth(1)
                            .unwrap()
                            .split("@")
                            .nth(1)
                            .unwrap();

                        let mut out_dir = crane.packages.clone();
                        out_dir.push("nuget");
                        println!("{}", out_dir.display());

                        if !out_dir.exists() {
                            create_dir_all(&out_dir).unwrap();
                        }

                        Command::new("nuget")
                            .current_dir(&out_dir)
                            .args(["install", &package_name, "-Version", &version])
                            .output()
                            .unwrap();
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
