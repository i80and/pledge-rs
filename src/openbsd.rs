extern crate libc;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr;

use Error;

pub fn pledge<'p, 'e, P, E>(promises: P, execpromises: E) -> Result<(), Error>
where P: Into<Option<&'p str>>, E: Into<Option<&'e str>> {
    extern "C" {
        fn pledge(promises: *const c_char, execpromises: *const c_char) -> c_int;
    }

    let promises = promises.into().map(|x| CString::new(x).map_err(Error::Promises));
    let execpromises = execpromises.into().map(|x| CString::new(x).map_err(Error::Execpromises));

    let promises = match promises {
        Some(Ok(ref result)) => Ok(result.as_ptr()),
        Some(Err(error)) => Err(error),
        None => Ok(ptr::null()),
    }?;

    let execpromises = match execpromises {
        Some(Ok(ref result)) => Ok(result.as_ptr()),
        Some(Err(error)) => Err(error),
        None => Ok(ptr::null()),
    }?;

    unsafe {
        return match pledge(promises, execpromises) {
            0 => Ok(()),
            _ => Err(Error::Other(*libc::__errno())),
        };
    }
}
