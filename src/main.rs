mod crane;

use crane::*;
use std::env::*;

fn main() {
    let crane = Crane::new();

    crane.create_dirs();

    match args().nth(1).as_deref() {
        Some("link") => crane.link(),
        Some("clean") => crane.clean(),
        Some(_) => {}
        None => {}
    }
}
