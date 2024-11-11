use serde::Deserialize;
use std::fs::*;
use std::io::BufReader;
use std::os::windows::fs::symlink_dir;
use std::path::{Path, PathBuf};
use std::process::Command;
use windows::{Win32::Foundation::HANDLE, Win32::UI::Shell::*};

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

#[derive(Deserialize, Debug)]
struct Manifest {
    packages: Vec<String>,
}

struct GitHub {
    owner: String,
    repo: String,
    branch: String,
}

impl GitHub {
    fn new(package: &str) -> Self {
        Self {
            owner: String::from(
                package
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split("/")
                    .nth(0)
                    .unwrap(),
            ),
            repo: String::from(
                package
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split("@")
                    .nth(0)
                    .unwrap()
                    .split("/")
                    .nth(1)
                    .unwrap(),
            ),
            branch: String::from(
                package
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split("@")
                    .nth(1)
                    .unwrap(),
            ),
        }
    }

    fn download<P: AsRef<Path>>(&self, out_dir: &P) {
        Command::new("gh")
            .current_dir(&out_dir)
            .args([
                "repo",
                "clone",
                format!("{}/{}", &self.owner, &self.repo).as_str(),
                format!("{}/{}", &self.repo, &self.branch).as_str(),
                "--",
                "--branch",
                &self.branch,
                "--depth=1",
            ])
            .output()
            .unwrap();
    }
}

struct Nuget {
    name: String,
    version: String,
}

impl Nuget {
    fn new(package: &str) -> Self {
        Self {
            name: String::from(
                package
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split("@")
                    .nth(0)
                    .unwrap(),
            ),
            version: String::from(
                package
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split("@")
                    .nth(1)
                    .unwrap(),
            ),
        }
    }

    fn download<P: AsRef<Path>>(&self, out_dir: &P) {
        Command::new("nuget")
            .current_dir(&out_dir)
            .args(["install", &self.name, "-Version", &self.version])
            .output()
            .unwrap();
    }
}

#[derive(Debug)]
struct Crane {
    root: PathBuf,
    packages: PathBuf,
    manifest: PathBuf,
    links: PathBuf,
}

impl Crane {
    fn new() -> Self {
        let root = PathBuf::from(app_data()).join("crane");
        let packages = root.clone().join("packages");

        Self {
            root: root,
            packages: packages,
            manifest: PathBuf::from("crane.json"),
            links: std::env::current_dir().unwrap().join("crane_packages"),
        }
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

    match File::open(&crane.manifest) {
        Ok(manifest_file) => {
            let reader = BufReader::new(manifest_file);
            let manifest = serde_json::from_reader::<_, Manifest>(reader).unwrap();

            for package in manifest.packages.iter() {
                let provider = package.split(":").nth(0).unwrap();

                match provider {
                    "gh" => {
                        let gh = GitHub::new(package);

                        let mut out_dir = crane.packages.clone();
                        out_dir.push("gh");
                        out_dir.push(&gh.owner);

                        if !out_dir.exists() {
                            create_dir_all(&out_dir).unwrap();
                        }

                        gh.download(&out_dir);

                        out_dir.push(&gh.repo);
                        out_dir.push(&gh.branch);

                        let mut link = crane.links.clone();
                        link.push(&gh.repo);

                        if !link.exists() {
                            symlink_dir(&out_dir, &link).unwrap();
                        }
                    }
                    "nuget" => {
                        let nuget = Nuget::new(package);

                        let mut out_dir = crane.packages.clone();
                        out_dir.push("nuget");

                        if !out_dir.exists() {
                            create_dir_all(&out_dir).unwrap();
                        }

                        nuget.download(&out_dir);

                        let id = format!("{}.{}", &nuget.name, &nuget.version);
                        out_dir.push(&id);

                        let mut link = crane.links.clone();
                        link.push(&id);

                        if !link.exists() {
                            symlink_dir(&out_dir, &link).unwrap();
                        }
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
