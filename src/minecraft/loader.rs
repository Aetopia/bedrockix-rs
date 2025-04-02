use std::mem::transmute;
use std::os::raw::c_void;
use std::{fs, ptr};
use std::{path::absolute, sync::LazyLock};
use windows::Win32::Foundation::{CloseHandle, GENERIC_ALL};
use windows::Win32::Security::Authorization::{
    EXPLICIT_ACCESS_W, SE_FILE_OBJECT, SET_ACCESS, SetEntriesInAclW, SetNamedSecurityInfoW,
    TRUSTEE_IS_NAME, TRUSTEE_IS_WELL_KNOWN_GROUP, TRUSTEE_W,
};
use windows::Win32::Security::SUB_CONTAINERS_AND_OBJECTS_INHERIT;
use windows::Win32::Security::{ACL, DACL_SECURITY_INFORMATION};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Memory::*;
use windows::Win32::System::Threading::{CreateRemoteThread, INFINITE, WaitForSingleObject};
use windows::core::{Error, Result};

use crate::windows::{Access, Process, WSTR};
use crate::windows::{CWSTR, procedure::Procedure};

use super::Game;

static PROCEDURE: LazyLock<Procedure> =
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

pub fn load(process: &Process, value: &str) -> Result<()> {
    if let Some(value) = absolute(value)?.to_str() {
        if !fs::exists(value)? {
            return Err(Error::empty());
        }

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

            let size = std::mem::size_of::<u16>() * (value.len() + 1);
            let parameter = VirtualAllocEx(
                process.handle,
                None,
                size,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            );

            _ = WriteProcessMemory(
                process.handle,
                parameter,
                path.0.as_ptr() as *const c_void,
                size,
                None,
            );

            if let Ok(thread) = CreateRemoteThread(
                process.handle,
                None,
                0,
                Some(transmute(PROCEDURE.0)),
                Some(parameter as *mut c_void),
                0,
                None,
            ) {
                WaitForSingleObject(thread, INFINITE);
                _ = CloseHandle(thread);
            }

            _ = VirtualFreeEx(process.handle, parameter, 0, MEM_RELEASE);
        }
    }

    Ok(())
}

pub struct Loader;

impl Loader {
    pub fn launch(value: &str) -> Result<u32> {
        let process = Game::activate()?;
        load(&process, value)?;
        Ok(process.id)
    }
}
