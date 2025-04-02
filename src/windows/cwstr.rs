use windows::core::PCWSTR;

pub struct CWSTR(pub PCWSTR);

unsafe impl Send for CWSTR {}

unsafe impl Sync for CWSTR {}

impl CWSTR {
    pub fn new(value: &str) -> Self {
        let vector: Vec<u16> = value.encode_utf16().collect();
        Self(PCWSTR(vector.first().unwrap()))
    }
}
