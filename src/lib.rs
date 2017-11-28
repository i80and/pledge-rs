use std::os::raw::c_int;
use std::{error, fmt};

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    UnsupportedPlatform,
    Other(c_int),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnsupportedPlatform => write!(f, "pledge is unsupported on this platform"),
            Error::Other(errno) => write!(f, "unable to pledge ({})", errno),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnsupportedPlatform => "pledge is unsupported on this platform",
            Error::Other(_) => "unable to pledge",
        }
    }
}

pub enum Promise {
    Audio,
    Bpf,
    Chown,
    CPath,
    DiskLabel,
    Dns,
    DPath,
    Drm,
    Exec,
    Fattr,
    Flock,
    Getpw,
    Id,
    Inet,
    Ioctl,
    MCast,
    Pf,
    Proc,
    ProtExec,
    Ps,
    Recvfd,
    Route,
    RPath,
    Sendfd,
    Settime,
    Stdio,
    Tape,
    TMPPath,
    Tty,
    Unix,
    Vminfo,
    Vmm,
    WPath,
}

impl Promise {
    pub fn to_promise_string(&self) -> &'static str {
        match *self {
            Promise::Audio => "audio",
            Promise::Bpf => "bpf",
            Promise::Chown => "chown",
            Promise::CPath => "cpath",
            Promise::DiskLabel => "disklabel",
            Promise::Dns => "dns",
            Promise::DPath => "dpath",
            Promise::Drm => "drm",
            Promise::Exec => "exec",
            Promise::Fattr => "fattr",
            Promise::Flock => "flock",
            Promise::Getpw => "getpw",
            Promise::Id => "id",
            Promise::Inet => "inet",
            Promise::Ioctl => "ioctl",
            Promise::MCast => "mcast",
            Promise::Pf => "pf",
            Promise::Proc => "proc",
            Promise::ProtExec => "prot_exec",
            Promise::Ps => "ps",
            Promise::Recvfd => "recvfd",
            Promise::Route => "route",
            Promise::RPath => "rpath",
            Promise::Sendfd => "sendfd",
            Promise::Settime => "settime",
            Promise::Stdio => "stdio",
            Promise::Tape => "tape",
            Promise::TMPPath => "tmppath",
            Promise::Tty => "tty",
            Promise::Unix => "unix",
            Promise::Vminfo => "vminfo",
            Promise::Vmm => "vmm",
            Promise::WPath => "wpath",
        }
    }
}

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

#[cfg(any(target_os = "bitrig", target_os = "openbsd"))]
mod openbsd;

#[cfg(any(target_os = "bitrig", target_os = "openbsd"))]
pub use openbsd::pledge_with_paths;

#[cfg(not(any(target_os = "bitrig", target_os = "openbsd")))]
pub fn pledge_with_paths(_: &str, _: &[&std::path::Path]) -> Result<(), Error> {
    Err(Error::UnsupportedPlatform)
}

pub fn pledge(promises: &str) -> Result<(), Error> {
    pledge_with_paths(promises, &[])
}

#[macro_export]
macro_rules! pledge {
    ( $( $x:ident ),* ) => {
        {
            let mut promises = Vec::new();
            $(
                promises.push(Promise::$x);
            )*
            let promises_str = promises.to_promise_string();
            pledge(&promises_str)
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
    #[cfg(not(any(target_os = "bitrig", target_os = "openbsd")))]
    fn test_pledge_unsupported() {
        use super::Error;
        assert_eq!(pledge![Stdio].unwrap_err(), Error::UnsupportedPlatform);
    }

    #[test]
    #[cfg(any(target_os = "bitrig", target_os = "openbsd"))]
    fn test_pledge_supported() {
        pledge![Stdio].unwrap();
        assert!(pledge![Stdio, Audio].is_err());
    }

    #[test]
    #[cfg(target_os = "openbsd")]
    fn test_as_string() {
        if pledge("stdio").is_err() {
            panic!("pledge");
        }
        println!("hello world");
    }
}
