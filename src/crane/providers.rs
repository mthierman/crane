use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use url::Url;
use windows::core::HSTRING;
use windows::Win32::System::Com::Urlmon::URLDownloadToFileW;

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
