use std::ffi::c_char;

pub extern "C" fn systemd_version() -> *const c_char {
    return b"257.5".as_ptr() as *const c_char
}