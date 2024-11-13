mod crane;

// use crane::*;
use crate::crane::*;
use std::env::*;
use std::fs::*;

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

    match args().nth(1).as_deref() {
        Some("link") => link(&crane),
        Some("clean") => clean(&crane),
        Some(_) => {}
        None => {}
    }
}
