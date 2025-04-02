use std::path::{Path, absolute};

use windows::Win32::{Foundation::FreeLibrary, Storage::FileSystem::GetBinaryTypeW};

use super::WString;

pub struct Library<'a> {
    exists: bool,
    valid: bool,
    path: &'a str,
}

impl Library<'_> {
    pub fn new(value: &Path) {
        let path = absolute(value).unwrap().display().to_string();
        let this = Path::new(&path);
        let exists = this.exists() && this.extension().is_some();
        let mut valid = false;

        if exists {
            unsafe {
                let mut binarytype = 0u32;
             _ =   GetBinaryTypeW(WString::new(&path).0, &mut binarytype).is_err() && FreeLibrary(LoadLi)
            }
        }
    }
}
