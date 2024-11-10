use std::path::PathBuf;
use windows::{Win32::Foundation::*, Win32::UI::Shell::*};

pub fn app_data() -> PathBuf {
    // let path: PathBuf;
    let result: String;

    unsafe {
        result = SHGetKnownFolderPath(
            &FOLDERID_LocalAppData,
            KNOWN_FOLDER_FLAG::default(),
            HANDLE::default(),
        )
        .unwrap()
        .to_string()
        .unwrap();
    };

    [result.as_str(), "crane"].iter().collect()
}
