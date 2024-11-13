use std::path::Path;
use std::process::Command;

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
