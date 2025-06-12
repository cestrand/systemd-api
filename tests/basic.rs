use std::ffi::CStr;

use systemd_api::{self, systemd_version};

#[test]
fn test_version() {
    let version = {
        let ret = unsafe {CStr::from_ptr(systemd_version())};
        ret.to_str().unwrap()
    };
    assert_eq!("257.5", version)
}