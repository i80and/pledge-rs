mod promise;

use std::ffi::NulError;
use std::os::raw::c_int;
use std::{error, fmt};

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    UnsupportedPlatform,
    Promises(NulError),
    Execpromises(NulError),
    Other(c_int),
}

impl Error {
    pub fn ignore_platform(self) -> Result<(), Self> {
        match self {
            Error::UnsupportedPlatform => Ok(()),
            x => Err(x),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnsupportedPlatform => write!(f, "pledge is unsupported on this platform"),
            Error::Promises(_) => write!(f, "unexpected NUL character in promises argument"),
            Error::Execpromises(_) => write!(f, "unexpected NUL character in execpromises argument"),
            Error::Other(errno) => write!(f, "unable to pledge ({})", errno),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnsupportedPlatform => "pledge is unsupported on this platform",
            Error::Promises(_) => "unexpected NUL character in promises argument",
            Error::Execpromises(_) => "unexpected NUL character in execpromises argument",
            Error::Other(_) => "unable to pledge",
        }
    }
}

pub use promise::Promise;

pub trait ToPromiseString {
    fn to_promise_string(&self) -> String;
}

impl ToPromiseString for [Promise] {
    fn to_promise_string(&self) -> String {
        self.iter()
            .map(|p| p.to_promise_string())
            .collect::<Vec<&'static str>>()
            .join(" ")
    }
}

#[cfg(target_os = "openbsd")]
mod openbsd;

#[cfg(target_os = "openbsd")]
pub use openbsd::pledge;

#[cfg(not(target_os = "openbsd"))]
pub fn pledge<'p, 'e, P, E>(_: P, _: E) -> Result<(), Error>
where P: Into<Option<&'p str>>, E: Into<Option<&'e str>> {
    Err(Error::UnsupportedPlatform)
}

#[macro_export]
macro_rules! pledge {
    [$($promises:ident)*, $($execpromises:ident)*] => {
        {
            let mut promises = Vec::new();
            let mut execpromises = Vec::new();
            $(
                promises.push($crate::Promise::$promises);
            )*
            $(
                execpromises.push($crate::Promise::$execpromises);
            )*
            let promises = $crate::ToPromiseString::to_promise_string(&*promises);
            let execpromises = $crate::ToPromiseString::to_promise_string(&*execpromises);
            $crate::pledge(&*promises, &*execpromises)
        }
    };
}

#[macro_export]
macro_rules! pledge_promises {
    [$($promises:ident)*] => {
        {
            let mut promises = Vec::new();
            $(
                promises.push($crate::Promise::$promises);
            )*
            let promises = $crate::ToPromiseString::to_promise_string(&*promises);
            $crate::pledge(&*promises, None)
        }
    };
}

#[macro_export]
macro_rules! pledge_execpromises {
    [$($execpromises:ident)*] => {
        {
            let mut execpromises = Vec::new();
            $(
                execpromises.push($crate::Promise::$execpromises);
            )*
            let execpromises = $crate::ToPromiseString::to_promise_string(&*execpromises);
            $crate::pledge(None, &*execpromises)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::{pledge, Promise, ToPromiseString};

    #[test]
    fn test_promise_str() {
        use super::ToPromiseString;

        assert_eq!(vec![].to_promise_string(), "");
        assert_eq!(vec![Promise::Dns].to_promise_string(), "dns");
        assert_eq!(
            vec![Promise::Stdio, Promise::ProtExec].to_promise_string(),
            "stdio prot_exec"
        );
    }

    #[test]
    #[cfg(not(target_os = "openbsd"))]
    fn test_pledge_unsupported() {
        use super::Error;
        assert_eq!(pledge_promises![Stdio].unwrap_err(), Error::UnsupportedPlatform);
    }

    #[test]
    #[cfg(target_os = "openbsd")]
    fn test_pledge_supported() {
        pledge_promises![Stdio].unwrap();
        assert!(pledge_promises![Stdio Audio].is_err());
    }

    #[test]
    #[cfg(target_os = "openbsd")]
    fn test_as_string() {
        pledge("stdio", None).unwrap();
        println!("hello world");
    }
}
