use std::path::Path;
use std::process::Command;

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
            .current_dir(out_dir)
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
            .current_dir(out_dir)
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
            .current_dir(out_dir)
            .args(["pull", "--depth=1", "--recurse-submodules"])
            .output()
            .unwrap();
    }
}
