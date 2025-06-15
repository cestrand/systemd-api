#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
pub mod version;

include!("bindings.rs");

use crate::version::{systemctl_version, Version};

#[unsafe(no_mangle)]
pub extern "C" fn c_systemd_version() -> *mut Version {
    match systemctl_version() {
        Ok(version) => Box::into_raw(Box::new(version)),
        _ => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn c_systemd_version_free(ptr: *mut Version) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) }
    }
}