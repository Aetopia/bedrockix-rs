use windows::core::{PCSTR, PCWSTR};

pub struct CWSTR(pub PCWSTR);

unsafe impl Send for CWSTR {}

unsafe impl Sync for CWSTR {}

pub struct CSTR(pub PCSTR);

unsafe impl Send for CSTR {}

unsafe impl Sync for CSTR {}

impl CWSTR {
    pub fn new(value: &str) -> Self {
        let bytes: Vec<u16> = value.encode_utf16().collect();
        Self(PCWSTR(bytes.first().unwrap()))
    }
}

impl CSTR {
    pub fn new(value: &str) -> Self {
        let bytes = value.as_bytes();
        Self(PCSTR(bytes.first().unwrap()))
    }
}
