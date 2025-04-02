use std::ops::Deref;
use windows::{
    Win32::System::Com::{
        CLSCTX_INPROC_SERVER, COINIT_DISABLE_OLE1DDE, COINIT_MULTITHREADED, CoCreateInstance,
        CoGetContextToken, CoInitializeEx,
    },
    core::{GUID, IUnknown, Interface, Result},
};

pub struct Com<T: Interface> {
    value: T,
}

unsafe impl<T: Interface> Sync for Com<T> {}

unsafe impl<T: Interface> Send for Com<T> {}

impl<T: Interface> Com<T> {
    pub fn create(rclsid: *const GUID) -> Result<Com<T>> {
        unsafe {
            if CoGetContextToken().is_err() {
                _ = CoInitializeEx(None, COINIT_DISABLE_OLE1DDE | COINIT_MULTITHREADED);
            }

            CoCreateInstance::<Option<&IUnknown>, T>(rclsid, None, CLSCTX_INPROC_SERVER)
                .map(|value| Self { value })
        }
    }
}

impl<T: Interface> Deref for Com<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
