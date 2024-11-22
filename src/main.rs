use crane::*;
use windows::Win32::UI::Shell::*;

fn main() {
    println!("{}", common::known_folder(&FOLDERID_LocalAppData).display());

    let crane = Crane::new();

    crane.run();
}
