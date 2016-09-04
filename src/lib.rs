use std::os::raw::c_int;

#[derive(PartialEq,Eq,Debug)]
pub enum Error {
    UnsupportedPlatform,
    Other(c_int),
}

pub enum Promise {
    Stdio,
    RPath,
    WPath,
    CPath,
    DPath,
    TMPPath,
    Inet,
    Fattr,
    Chown,
    Flock,
    Unix,
    Dns,
    Getpw,
    Sendfd,
    Recvfd,
    Ioctl,
    Tty,
    Proc,
    Exec,
    ProtExec,
    Settime,
    Ps,
    Vminfo,
    Id,
    Pf,
    Audio,
}

impl Promise {
    pub fn to_promise_string(&self) -> &'static str {
        return match *self {
            Promise::Stdio => "stdio",
            Promise::RPath => "rpath",
            Promise::WPath => "wpath",
            Promise::CPath => "cpath",
            Promise::DPath => "dpath",
            Promise::TMPPath => "tmppath",
            Promise::Inet => "inet",
            Promise::Fattr => "fattr",
            Promise::Chown => "chown",
            Promise::Flock => "flock",
            Promise::Unix => "unix",
            Promise::Dns => "dns",
            Promise::Getpw => "getpw",
            Promise::Sendfd => "sendfd",
            Promise::Recvfd => "recvfd",
            Promise::Ioctl => "ioctl",
            Promise::Tty => "tty",
            Promise::Proc => "proc",
            Promise::Exec => "exec",
            Promise::ProtExec => "prot_exec",
            Promise::Settime => "settime",
            Promise::Ps => "ps",
            Promise::Vminfo => "vminfo",
            Promise::Id => "id",
            Promise::Pf => "pf",
            Promise::Audio => "audio",
        };
    }
}

trait ToPromiseString {
    fn to_promise_string(&self) -> String;
}

impl ToPromiseString for [Promise] {
    fn to_promise_string(&self) -> String {
        return self.iter()
            .map(|p| p.to_promise_string())
            .collect::<Vec<&'static str>>()
            .join(" ");
    }
}

#[cfg(target_os = "openbsd")]
mod openbsd;

#[cfg(target_os = "openbsd")]
pub use openbsd::pledge_wrapper as pledge;

#[cfg(not(target_os = "openbsd"))]
pub fn pledge(_: &[Promise]) -> Result<(), Error> {
    return Err(Error::UnsupportedPlatform);
}

#[macro_export]
macro_rules! pledge {
    ( $( $x:ident ),* ) => {
        {
            let mut promises = Vec::new();
            $(
                promises.push(Promise::$x);
            )*
            pledge(&promises)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::{Promise, pledge};

    #[test]
    fn test_promise_str() {
        use super::{ToPromiseString};

        assert_eq!(vec![].to_promise_string(), "");
        assert_eq!(vec![Promise::Dns].to_promise_string(), "dns");
        assert_eq!(vec![Promise::Stdio, Promise::ProtExec].to_promise_string(),
                   "stdio prot_exec");
    }

    #[test]
    #[cfg(not(target_os = "openbsd"))]
    fn test_pledge_unsupported() {
        use super::Error;
        assert_eq!(pledge![Stdio].unwrap_err(), Error::UnsupportedPlatform);
    }

    #[test]
    #[cfg(target_os = "openbsd")]
    fn test_pledge_supported() {
        pledge![Stdio].unwrap();
        assert!(pledge![Stdio, Audio].is_err());
    }
}
