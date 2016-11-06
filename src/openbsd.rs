extern crate libc;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr;

use Error;

pub fn pledge(promises: &str) -> Result<(), Error> {
    extern "C" {
        fn pledge(promises: *const c_char, paths: *const *const c_char) -> c_int;
    }

    let cstr = CString::new(promises).unwrap();

    unsafe {
        return match pledge(cstr.as_ptr(), ptr::null()) {
            0 => Ok(()),
            _ => Err(Error::Other(*libc::__errno())),
        };
    }
}
