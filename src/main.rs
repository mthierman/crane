use crane::*;

fn main() {
    let crane = Crane::new();

    crane.create_dirs();
    crane.parse_args();
}
