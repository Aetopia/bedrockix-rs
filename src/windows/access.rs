use windows::Win32::Security::Authorization::EXPLICIT_ACCESS_W;

pub struct Access(pub EXPLICIT_ACCESS_W);

unsafe impl Sync for Access {}

unsafe impl Send for Access {}