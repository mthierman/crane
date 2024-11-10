// https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
// https://docs.rs/serde_json/latest/serde_json/fn.from_reader.html
// https://doc.rust-lang.org/rust-by-example/flow_control/for.html

use serde::Deserialize;
use std::fs::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Command;

// use windows::{Win32::Foundation::*, Win32::UI::Shell::*};

#[derive(Deserialize, Debug)]
struct Manifest {
    packages: Vec<String>,
}

fn main() {
    // let result: String;

    // unsafe {
    //     result = SHGetKnownFolderPath(
    //         &FOLDERID_LocalAppData,
    //         KNOWN_FOLDER_FLAG::default(),
    //         HANDLE::default(),
    //     )
    //     .unwrap()
    //     .to_string()
    //     .unwrap();
    // };

    // let path: PathBuf = [result.as_str(), "crane", "crane.json"].iter().collect();

    let path = PathBuf::from("crane.json");
    let output_directory = PathBuf::from("crane_packages");
    let _ = create_dir(output_directory).unwrap();

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let u = serde_json::from_reader::<_, Manifest>(reader).unwrap();

    for package in u.packages.iter() {
        let split: Vec<&str> = package.split(":").collect();

        match split[0] {
            "gh" => {
                // let owner = split[1].split("/").nth(0).unwrap();

                // let repo = split[1]
                //     .split("/")
                //     .nth(1)
                //     .unwrap()
                //     .split("@")
                //     .nth(0)
                //     .unwrap();

                // let branch = split[1]
                //     .split("/")
                //     .nth(1)
                //     .unwrap()
                //     .split("@")
                //     .nth(1)
                //     .unwrap();

                // println!("{} - {} - {}", owner, repo, branch);

                let repo = split[1].split("@").next().unwrap();
                let branch = split[1].split("@").nth(1).unwrap();
                println!("{} - {}", repo, branch);

                let output = Command::new("gh")
                    .current_dir("crane_packages")
                    .args(["repo", "clone", repo, "--", "--branch", branch, "--depth=1"])
                    .output()
                    .unwrap();
                let printout = String::from_utf8(output.stdout).unwrap();
                println!("{}", printout);

                // let clone_url = "https://github.com/nlohmann/json.git";
                // let mut clone_url = String::from("https://github.com/");
                // clone_url.push_str(owner);
                // clone_url.push_str("/");
                // clone_url.push_str(repo);
                // clone_url.push_str(".git");
                // println!("{}", clone_url);
                // let output = Command::new("git").args(["status"]).output().unwrap();
                // let printout = String::from_utf8(output.stdout).unwrap();
                // println!("{}", printout);
            }
            "nuget" => {
                let package = split[1].split("@").next().unwrap();
                let version = split[1].split("@").nth(1).unwrap();
                println!("{} - {}", package, version);

                let output = Command::new("nuget")
                    .current_dir("crane_packages")
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