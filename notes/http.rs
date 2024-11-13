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
