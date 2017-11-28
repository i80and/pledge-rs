extern crate libc;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::os::unix::ffi::OsStrExt;
use std::path;
use std::ptr;

use Error;

pub fn pledge_with_paths(promises: &str, paths: &[&path::Path]) -> Result<(), Error> {
    extern "C" {
        fn pledge(promises: *const c_char, paths: *const *const c_char) -> c_int;
    }

    let cstr = CString::new(promises).unwrap();
    let mut cpaths = Vec::with_capacity(paths.len());
    for path in paths {
        cpaths.push(CString::new(path.as_os_str().as_bytes()).unwrap());
    }

    let mut cpaths_raw = cpaths
        .iter()
        .map(|path| path.as_ptr())
        .collect::<Vec<*const libc::c_char>>();

    unsafe {
        let cpaths_ptr = if !cpaths.is_empty() {
            cpaths_raw.push(ptr::null());
            cpaths_raw.as_ptr()
        } else {
            ptr::null()
        };

        return match pledge(cstr.as_ptr(), cpaths_ptr) {
            0 => Ok(()),
            _ => Err(Error::Other(*libc::__errno())),
        };
    }
}
