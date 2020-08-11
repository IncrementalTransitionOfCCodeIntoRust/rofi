/***
 * public functions
 ***/

use std::ffi::CString;

pub fn to_const_i8(s: &str) -> *const i8 {
    CString::new(s).unwrap().as_ptr() as *const i8
}
