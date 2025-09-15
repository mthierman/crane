mod providers;

use providers::*;
use serde::Deserialize;
use std::env::*;
use std::fs::*;
use std::io::BufReader;
use std::os::windows::fs::symlink_dir;
use std::path::PathBuf;
use windows::Win32::UI::Shell::*;

#[derive(Deserialize, Debug)]
struct Manifest {
    packages: Vec<String>,
}

#[derive(Debug)]
struct Paths {
    manifest: PathBuf,
    packages: PathBuf,
}

#[derive(Debug)]
pub struct Crane {
    paths: Paths,
}

impl Crane {
    pub fn init() -> Self {
        let app_data = unsafe {
            SHGetKnownFolderPath(&FOLDERID_LocalAppData, KF_FLAG_DONT_VERIFY, None)
                .unwrap()
                .to_string()
                .unwrap()
        };

        let data = PathBuf::from(app_data).join("crane");

        if !data.exists() {
            create_dir_all(&data).unwrap();
        }

        let cache = data.clone().join("cache");

        if !cache.exists() {
            create_dir_all(&cache).unwrap();
        }

        let packages = std::env::current_dir().unwrap().join("crane_packages");

        if !packages.exists() {
            create_dir_all(&packages).unwrap();
        }

        Self {
            paths: Paths {
                manifest: PathBuf::from("crane.json"),
                packages: std::env::current_dir().unwrap().join("crane_packages"),
            },
        }
    }

    pub fn run(&self) {
        match args().nth(1).as_deref() {
            Some("link") => self.link(),
            Some("clean") => self.clean(),
            Some("manifest") => self.print_manifest(),
            Some(_) => {}
            None => {}
        }
    }

    fn read_manifest(&self) -> Option<Manifest> {
        match File::open(&self.paths.manifest) {
            Ok(file) => {
                let reader = BufReader::new(file);
                Some(serde_json::from_reader::<_, Manifest>(reader).unwrap())
            }
            Err(_) => {
                match File::create(&self.paths.manifest) {
                    Ok(_) => {
                        println!("Manifest created");
                    }
                    Err(e) => {
                        println!("Manifest creation failed: {}", e);
                    }
                }
                None
            }
        }
    }

    pub fn link(&self) {
        if let Some(manifest) = self.read_manifest() {
            for package in manifest.packages.iter() {
                match package.split(":").nth(0) {
                    Some("http") | Some("https") => {
                        let http = Http::new(package);

                        let mut out_dir = self.paths.packages.clone();
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

                        let mut out_dir = self.paths.packages.clone();
                        out_dir.push("gh");
                        out_dir.push(&gh.owner);

                        if !out_dir.exists() {
                            create_dir_all(&out_dir).unwrap();
                            gh.download(&out_dir);
                        }

                        out_dir.push(&gh.repo);
                        out_dir.push(&gh.branch);

                        let mut link = self.paths.packages.clone();
                        link.push(&gh.repo);

                        if !link.exists() {
                            symlink_dir(&out_dir, &link).unwrap();
                        }

                        gh.update(&out_dir);
                    }
                    Some("nuget") => {
                        let nuget = Nuget::new(package);

                        let mut out_dir = self.paths.packages.clone();
                        out_dir.push("nuget");

                        if !out_dir.exists() {
                            create_dir_all(&out_dir).unwrap();
                        }

                        nuget.download(&out_dir);

                        let id = format!("{}.{}", &nuget.name, &nuget.version);
                        out_dir.push(&id);

                        let mut link = self.paths.packages.clone();
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
    }

    pub fn clean(&self) {
        remove_dir_all(&self.paths.packages).unwrap();
    }

    pub fn print_manifest(&self) {
        if let Ok(manifest) = read_to_string(&self.paths.manifest) {
            println!("{}", manifest);
        }
    }
}
