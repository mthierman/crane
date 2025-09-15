use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use url::Url;
use windows::core::HSTRING;
use windows::Win32::System::Com::Urlmon::URLDownloadToFileW;

pub struct Http {
    pub url: Url,
    pub extension: String,
}

impl Http {
    pub fn new(package: &str) -> Self {
        let url = Url::parse(package).unwrap();
        let extension = PathBuf::from(&url.path_segments().unwrap().next_back().unwrap())
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        Self { url, extension }
    }

    pub fn zip<P: AsRef<Path>>(&self, out_dir: &P) {
        Command::new("7z")
            .current_dir(out_dir)
            .args([
                "x",
                self.url.path_segments().unwrap().next_back().unwrap(),
                PathBuf::from(self.url.path_segments().unwrap().next_back().unwrap())
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            ])
            .output()
            .unwrap();
    }

    pub fn tar_xz<P: AsRef<Path>>(&self, out_dir: &P) {
        Command::new("tar")
            .current_dir(out_dir)
            .args([
                "-xf",
                self.url.path_segments().unwrap().next_back().unwrap(),
            ])
            .output()
            .unwrap();
    }

    pub fn download<P: AsRef<Path>>(&self, out_dir: &P) {
        let out_file = out_dir
            .as_ref()
            .to_path_buf()
            .join(self.url.path_segments().unwrap().next_back().unwrap());

        unsafe {
            URLDownloadToFileW(
                None,
                &HSTRING::from(&self.url.to_string()),
                &HSTRING::from(out_file.to_str().unwrap()),
                0,
                None,
            )
            .expect("URLDownloadToFileW");
        }
    }
}
