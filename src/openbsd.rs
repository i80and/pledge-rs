extern crate libc;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr;

use {Promise, Error, ToPromiseString};

extern "C" {
    fn pledge(promises: *const c_char, paths: *const *const c_char) -> c_int;
}

pub fn pledge_wrapper(promises: &[Promise]) -> Result<(), Error> {
    let promise_str = promises.to_promise_string();
    let cstr = CString::new(promise_str).unwrap();

    unsafe {
        return match pledge(cstr.as_ptr(), ptr::null()) {
            0 => Ok(()),
            _ => Err(Error::Other(*libc::__erno())),
        };
    }
}
