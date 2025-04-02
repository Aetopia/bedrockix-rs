use std::{
    ffi::{OsStr, OsString},
    os::windows::ffi::OsStrExt,
};

use windows::core::{PCSTR, PCWSTR, PWSTR};

pub struct CWSTR(pub PCWSTR, Vec<u16>);

unsafe impl Send for CWSTR {}

unsafe impl Sync for CWSTR {}

impl CWSTR {
    pub fn new(value: &str) -> Self {
        let mut vector: Vec<u16> = value.encode_utf16().collect();
        vector.push(0);
       
        Self {
            0: PCWSTR(vector.as_ptr()),
            1: vector,
        }
    }
}

pub struct WSTR(pub PWSTR, Vec<u16>);

unsafe impl Send for WSTR {}

unsafe impl Sync for WSTR {}

impl WSTR {
    pub fn new(value: &str) -> Self {
        let mut vector: Vec<u16> = value.encode_utf16().collect();
        vector.push(0);

        Self {
            0: PWSTR(vector.as_mut_ptr()),
            1: vector,
        }
    }
}

pub struct CSTR(pub PCSTR, Vec<u8>);

unsafe impl Send for CSTR {}

unsafe impl Sync for CSTR {}

impl CSTR {
    pub fn new(value: &str) -> Self {
        let mut vector = value.as_bytes().to_vec();
        vector.push(0);
    
        Self {
            0: PCSTR(vector.as_ptr()),
            1: vector,
        }
    }
}
