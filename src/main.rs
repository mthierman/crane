mod crane;

use crane::*;
use std::env::*;

fn main() {
    let crane = Crane::new();

    crane.create_dirs();

    match args().nth(1).as_deref() {
        Some("link") => link(&crane),
        Some("clean") => clean(&crane),
        Some(_) => {}
        None => {}
    }
}
