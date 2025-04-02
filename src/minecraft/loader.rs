use std::ffi::OsString;
use std::ptr;
use std::{path::absolute, sync::LazyLock};
use windows::Win32::Foundation::GENERIC_ALL;
use windows::Win32::Security::Authorization::{
    EXPLICIT_ACCESS_W, SE_FILE_OBJECT, SET_ACCESS, SetEntriesInAclW, SetNamedSecurityInfoW,
    TRUSTEE_IS_NAME, TRUSTEE_IS_WELL_KNOWN_GROUP, TRUSTEE_W,
};
use windows::Win32::Security::SUB_CONTAINERS_AND_OBJECTS_INHERIT;
use windows::Win32::Security::{ACL, DACL_SECURITY_INFORMATION};
use windows::core::{Error, PWSTR, Result, w};
use windows::{Win32::Foundation::HANDLE, core::PCWSTR};

use crate::windows::{Access, CSTR, WSTR};
use crate::windows::{CWSTR, procedure::Procedure};

static ADDRESS: LazyLock<Procedure> =
    LazyLock::new(|| Procedure::new("Kernel32", "LoadLibraryW").unwrap());

static NAME: LazyLock<WSTR> = LazyLock::new(|| WSTR::new("ALL APPLICATION PACKAGES"));

static ACCESS: LazyLock<Access> = LazyLock::new(|| {
    Access(EXPLICIT_ACCESS_W {
        grfAccessPermissions: GENERIC_ALL.0,
        grfAccessMode: SET_ACCESS,
        grfInheritance: SUB_CONTAINERS_AND_OBJECTS_INHERIT,
        Trustee: TRUSTEE_W {
            TrusteeForm: TRUSTEE_IS_NAME,
            TrusteeType: TRUSTEE_IS_WELL_KNOWN_GROUP,
            ptstrName: NAME.0,
            ..Default::default()
        },
        ..Default::default()
    })
});

pub fn load(value: &str) -> Result<()> {
    if let Some(value) = absolute(value)?.to_str() {
        let path = CWSTR::new(value);
        let mut acl: *mut ACL = ptr::null_mut();

        unsafe {
            _ = SetEntriesInAclW(Some(&[ACCESS.0]), None, &mut acl);
            _ = SetNamedSecurityInfoW(
                path.0,
                SE_FILE_OBJECT,
                DACL_SECURITY_INFORMATION,
                None,
                None,
                Some(acl),
                None,
            );
        }
    }

    Ok(())
}

pub struct Loader;

impl Loader {
    pub fn launch(value: &str) {
        _ = load(value)
    }
}
