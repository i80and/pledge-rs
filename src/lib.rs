use std::os::raw::c_int;

#[derive(PartialEq,Eq,Debug)]
pub enum Error {
    UnsupportedPlatform,
    Other(c_int),
}

pub enum Promise {
    Audio,
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
    TMPPath,
    Tty,
    Unix,
    Vminfo,
    Vmm,
    WPath,
}

impl Promise {
    pub fn to_promise_string(&self) -> &'static str {
        return match *self {
            Promise::Audio => "audio",
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
            Promise::TMPPath => "tmppath",
            Promise::Tty => "tty",
            Promise::Unix => "unix",
            Promise::Vminfo => "vminfo",
            Promise::Vmm => "vmm",
            Promise::WPath => "wpath",
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

#[cfg(any(target_os = "bitrig",
          target_os = "openbsd"))]
mod openbsd;

#[cfg(any(target_os = "bitrig",
          target_os = "openbsd"))]
pub use openbsd::pledge_with_paths;

#[cfg(not(any(target_os = "bitrig",
              target_os = "openbsd")))]
pub fn pledge_with_paths(_: &str, _: &[&std::path::Path]) -> Result<(), Error> {
    return Err(Error::UnsupportedPlatform);
}

pub fn pledge(promises: &str) -> Result<(), Error> {
    return pledge_with_paths(promises, &[]);
}

#[macro_export]
macro_rules! pledge {
    ( $( $x:ident ),* ) => {
        {
            use ToPromiseString;
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
    use super::{Promise, pledge};

    #[test]
    fn test_promise_str() {
        use super::ToPromiseString;

        assert_eq!(vec![].to_promise_string(), "");
        assert_eq!(vec![Promise::Dns].to_promise_string(), "dns");
        assert_eq!(vec![Promise::Stdio, Promise::ProtExec].to_promise_string(),
                   "stdio prot_exec");
    }

    #[test]
    #[cfg(not(any(target_os = "bitrig",
                  target_os = "openbsd")))]
    fn test_pledge_unsupported() {
        use super::Error;
        assert_eq!(pledge![Stdio].unwrap_err(), Error::UnsupportedPlatform);
    }

    #[test]
    #[cfg(any(target_os = "bitrig",
              target_os = "openbsd"))]
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
