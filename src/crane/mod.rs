mod providers;

use providers::*;
use serde::Deserialize;
use std::fs::*;
use std::io::BufReader;
use std::os::windows::fs::symlink_dir;
use std::path::PathBuf;
use windows::Win32::{Foundation::HANDLE, UI::Shell::*};

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub packages: Vec<String>,
}

#[derive(Debug)]
struct Paths {
    data: PathBuf,
    cache: PathBuf,
    manifest: PathBuf,
    packages: PathBuf,
}

#[derive(Debug)]
pub struct Crane {
    paths: Paths,
}

impl Crane {
    pub fn new() -> Self {
        let app_data = unsafe {
            SHGetKnownFolderPath(
                &FOLDERID_LocalAppData,
                KF_FLAG_DONT_VERIFY,
                HANDLE::default(),
            )
            .unwrap()
            .to_string()
            .unwrap()
        };
        let data = PathBuf::from(app_data).join("crane");
        let cache = data.clone().join("cache");

        Self {
            paths: Paths {
                cache: cache,
                data: data,
                manifest: PathBuf::from("crane.json"),
                packages: std::env::current_dir().unwrap().join("crane_packages"),
            },
        }
    }

    pub fn create_dirs(&self) {
        if !self.paths.data.exists() {
            create_dir_all(&self.paths.data).unwrap();
        }

        if !self.paths.cache.exists() {
            create_dir_all(&self.paths.cache).unwrap();
        }

        if !self.paths.packages.exists() {
            create_dir_all(&self.paths.packages).unwrap();
        }
    }
}

pub fn link(crane: &Crane) {
    match File::open(&crane.paths.manifest) {
        Ok(manifest_file) => {
            let reader = BufReader::new(manifest_file);
            let manifest = serde_json::from_reader::<_, Manifest>(reader).unwrap();

            for package in manifest.packages.iter() {
                match package.split(":").nth(0) {
                    Some("http") | Some("https") => {
                        let http = HTTP::new(package);

                        let mut out_dir = crane.paths.packages.clone();
                        out_dir.push("http");

                        if !out_dir.exists() {
                            create_dir_all(&out_dir).unwrap();
                        }

                        http.download(&out_dir);

                        match http.extension.as_str() {
                            "zip" => {
                                http.zip(&out_dir);
                            }
                            "xz" => {
                                http.tar_xz(&out_dir);
                            }
                            e => {
                                println!("{} file extension not supported", e);
                            }
                        }
                    }
                    Some("gh") => {
                        let gh = GitHub::new(package);

                        let mut out_dir = crane.paths.packages.clone();
                        out_dir.push("gh");
                        out_dir.push(&gh.owner);

                        if !out_dir.exists() {
                            create_dir_all(&out_dir).unwrap();
                            gh.download(&out_dir);
                        }

                        out_dir.push(&gh.repo);
                        out_dir.push(&gh.branch);

                        let mut link = crane.paths.packages.clone();
                        link.push(&gh.repo);

                        if !link.exists() {
                            symlink_dir(&out_dir, &link).unwrap();
                        }

                        gh.update(&out_dir);
                    }
                    Some("nuget") => {
                        let nuget = Nuget::new(package);

                        let mut out_dir = crane.paths.packages.clone();
                        out_dir.push("nuget");

                        if !out_dir.exists() {
                            create_dir_all(&out_dir).unwrap();
                        }

                        nuget.download(&out_dir);

                        let id = format!("{}.{}", &nuget.name, &nuget.version);
                        out_dir.push(&id);

                        let mut link = crane.paths.packages.clone();
                        link.push(&id);

                        if !link.exists() {
                            symlink_dir(&out_dir, &link).unwrap();
                        }
                    }
                    Some(e) => {
                        println!("Incorrect provider detected {}", e);
                    }
                    None => {}
                }
            }
        }
        Err(_) => {
            println!("Manifest doesn't exist, creating...");
            let _ = File::create(&crane.paths.manifest);
        }
    }
}

pub fn clean(crane: &Crane) {
    remove_dir_all(&crane.paths.packages).unwrap();
}
