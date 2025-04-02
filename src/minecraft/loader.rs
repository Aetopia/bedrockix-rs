use std::sync::LazyLock;
use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};
use windows::core::{s, w};
pub struct Loader;

struct Address(pub isize);
unsafe impl Send for Address {}
unsafe impl Sync for Address {}

static ADDRESS: LazyLock<Address> = LazyLock::new(|| unsafe {
    Address(
        GetProcAddress(
            GetModuleHandleW(w!("Kernel32")).unwrap(),
            s!("LoadLibraryW"),
        )
        .unwrap() as isize,
    )
});

impl Loader {
    pub fn launch() {
        unsafe {}
    }
}
