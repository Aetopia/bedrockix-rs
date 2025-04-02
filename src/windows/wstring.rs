use windows::core::PCWSTR;

pub struct WString(pub PCWSTR);

unsafe impl Send for WString {}

unsafe impl Sync for WString {}

impl WString {
    pub fn new(value: &str) -> Self {
        let string: Vec<u16> = value.encode_utf16().collect();
        Self(PCWSTR(string.first().unwrap()))
    }
}
