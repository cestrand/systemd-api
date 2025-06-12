#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
pub mod version;

include!("bindings.rs");

use std::ffi::c_char;

pub extern "C" fn systemd_version() -> *const c_char {
    return b"257.5\0".as_ptr() as *const c_char
}