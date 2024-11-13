use serde::Deserialize;
use std::fs::*;
use std::io::BufReader;
use std::os::windows::fs::symlink_dir;
use std::path::{Path, PathBuf};
use std::process::Command;
use url::Url;
use windows::core::HSTRING;
use windows::Win32::{Foundation::HANDLE, System::Com::Urlmon::URLDownloadToFileW, UI::Shell::*};

pub fn app_data() -> String {
    unsafe {
        SHGetKnownFolderPath(
            &FOLDERID_LocalAppData,
            KF_FLAG_DONT_VERIFY,
            HANDLE::default(),
        )
        .unwrap()
        .to_string()
        .unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub packages: Vec<String>,
}

pub struct GitHub {
    pub owner: String,
    pub repo: String,
    pub branch: String,
}

impl GitHub {
    pub fn new(package: &str) -> Self {
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

    #[allow(dead_code)]
    pub fn download_gh_cli<P: AsRef<Path>>(&self, out_dir: &P) {
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
                "--recurse-submodules",
            ])
            .output()
            .unwrap();
    }

    pub fn download<P: AsRef<Path>>(&self, out_dir: &P) {
        Command::new("git")
            .current_dir(&out_dir)
            .args([
                "clone",
                format!("https://github.com/{}/{}.git", &self.owner, &self.repo).as_str(),
                format!("{}/{}", &self.repo, &self.branch).as_str(),
                "--depth=1",
                "--recurse-submodules",
            ])
            .output()
            .unwrap();
    }

    pub fn update<P: AsRef<Path>>(&self, out_dir: &P) {
        Command::new("git")
            .current_dir(&out_dir)
            .args(["pull", "--depth=1", "--recurse-submodules"])
            .output()
            .unwrap();
    }
}

pub struct Nuget {
    pub name: String,
    pub version: String,
}

impl Nuget {
    pub fn new(package: &str) -> Self {
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

    pub fn download<P: AsRef<Path>>(&self, out_dir: &P) {
        Command::new("nuget")
            .current_dir(&out_dir)
            .args(["install", &self.name, "-Version", &self.version])
            .output()
            .unwrap();
    }
}

pub struct HTTP {
    pub url: Url,
    pub extension: String,
}

impl HTTP {
    pub fn new(package: &str) -> Self {
        let url = Url::parse(package).unwrap();
        let extension = PathBuf::from(&url.path_segments().unwrap().last().unwrap())
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        Self {
            url: url,
            extension: extension,
        }
    }

    pub fn zip<P: AsRef<Path>>(&self, out_dir: &P) {
        Command::new("7z")
            .current_dir(&out_dir)
            .args([
                "x",
                self.url.path_segments().unwrap().last().unwrap(),
                PathBuf::from(self.url.path_segments().unwrap().last().unwrap())
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            ])
            .output()
            .unwrap();
    }

    pub fn tar_xz<P: AsRef<Path>>(&self, out_dir: &P) {
        Command::new("tar")
            .current_dir(&out_dir)
            .args(["-xf", self.url.path_segments().unwrap().last().unwrap()])
            .output()
            .unwrap();
    }

    pub fn download<P: AsRef<Path>>(&self, out_dir: &P) {
        let out_file = out_dir
            .as_ref()
            .to_path_buf()
            .join(self.url.path_segments().unwrap().last().unwrap());

        unsafe {
            URLDownloadToFileW(
                None,
                &HSTRING::from(&self.url.to_string()),
                &HSTRING::from(out_file.to_str().unwrap()),
                0,
                None,
            )
            .expect("URLDownloadToFileW");
        }
    }
}

#[derive(Debug)]
pub struct Crane {
    pub root: PathBuf,
    pub packages: PathBuf,
    pub manifest: PathBuf,
    pub links: PathBuf,
}

impl Crane {
    pub fn new() -> Self {
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

pub fn link(crane: &Crane) {
    match File::open(&crane.manifest) {
        Ok(manifest_file) => {
            let reader = BufReader::new(manifest_file);
            let manifest = serde_json::from_reader::<_, Manifest>(reader).unwrap();

            for package in manifest.packages.iter() {
                match package.split(":").nth(0) {
                    Some("http") | Some("https") => {
                        let http = HTTP::new(package);

                        let mut out_dir = crane.packages.clone();
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

                        let mut out_dir = crane.packages.clone();
                        out_dir.push("gh");
                        out_dir.push(&gh.owner);

                        if !out_dir.exists() {
                            create_dir_all(&out_dir).unwrap();
                            gh.download(&out_dir);
                        }

                        out_dir.push(&gh.repo);
                        out_dir.push(&gh.branch);

                        let mut link = crane.links.clone();
                        link.push(&gh.repo);

                        if !link.exists() {
                            symlink_dir(&out_dir, &link).unwrap();
                        }

                        gh.update(&out_dir);
                    }
                    Some("nuget") => {
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
                    Some(e) => {
                        println!("Incorrect provider detected {}", e);
                    }
                    None => {}
                }
            }
        }
        Err(_) => {
            println!("Manifest doesn't exist, creating...");
            let _ = File::create(&crane.manifest);
        }
    }
}

pub fn clean(crane: &Crane) {
    remove_dir_all(&crane.links).unwrap();
}
