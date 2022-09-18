// This file was generated from the pledge(2) manual pages
// using the helper documented at </variants/README.md>.

pub enum Promise {
    /// OpenBSD 5.9–7.1
    Audio,

    /// OpenBSD 6.1–7.1
    Bpf,

    /// OpenBSD 6.0–7.1
    Chown,

    /// OpenBSD 5.9–7.1
    Cpath,

    /// OpenBSD 6.1–7.1
    Disklabel,

    /// OpenBSD 5.9–7.1
    Dns,

    /// OpenBSD 5.9–7.1
    Dpath,

    /// OpenBSD 6.1–7.1
    Drm,

    /// OpenBSD 6.3–7.1
    Error,

    /// OpenBSD 5.9–7.1
    Exec,

    /// OpenBSD 5.9–7.1
    Fattr,

    /// OpenBSD 5.9–7.1
    Flock,

    /// OpenBSD 5.9–7.1
    Getpw,

    /// OpenBSD 5.9–7.1
    Id,

    /// OpenBSD 5.9–7.1
    Inet,

    /// OpenBSD 5.9–6.0
    Ioctl,

    /// OpenBSD 6.1–7.1
    Mcast,

    /// OpenBSD 5.9–7.1
    Pf,

    /// OpenBSD 5.9–7.1
    Proc,

    /// OpenBSD 5.9–7.1
    ProtExec,

    /// OpenBSD 5.9–7.1
    Ps,

    /// OpenBSD 5.9–7.1
    Recvfd,

    /// OpenBSD 6.1–7.1
    Route,

    /// OpenBSD 5.9–7.1
    Rpath,

    /// OpenBSD 5.9–7.1
    Sendfd,

    /// OpenBSD 5.9–7.1
    Settime,

    /// OpenBSD 5.9–7.1
    Stdio,

    /// OpenBSD 6.1–7.1
    Tape,

    /// OpenBSD 5.9–7.1
    Tmppath,

    /// OpenBSD 5.9–7.1
    Tty,

    /// OpenBSD 5.9–7.1
    Unix,

    /// OpenBSD 6.4–7.1
    Unveil,

    /// OpenBSD 6.5–7.1
    Video,

    /// OpenBSD 5.9–7.1
    Vminfo,

    /// OpenBSD 6.1–7.1
    Vmm,

    /// OpenBSD 5.9–7.1
    Wpath,

    /// OpenBSD 6.8–7.1
    Wroute,

    // FIXME rust-lang/rust#44109
    #[doc(hidden)]
    _NonExhaustive,
}

impl Promise {
    pub fn to_promise_string(&self) -> &'static str {
        match *self {
            Promise::Audio => "audio",
            Promise::Bpf => "bpf",
            Promise::Chown => "chown",
            Promise::Cpath => "cpath",
            Promise::Disklabel => "disklabel",
            Promise::Dns => "dns",
            Promise::Dpath => "dpath",
            Promise::Drm => "drm",
            Promise::Error => "error",
            Promise::Exec => "exec",
            Promise::Fattr => "fattr",
            Promise::Flock => "flock",
            Promise::Getpw => "getpw",
            Promise::Id => "id",
            Promise::Inet => "inet",
            Promise::Ioctl => "ioctl",
            Promise::Mcast => "mcast",
            Promise::Pf => "pf",
            Promise::Proc => "proc",
            Promise::ProtExec => "prot_exec",
            Promise::Ps => "ps",
            Promise::Recvfd => "recvfd",
            Promise::Route => "route",
            Promise::Rpath => "rpath",
            Promise::Sendfd => "sendfd",
            Promise::Settime => "settime",
            Promise::Stdio => "stdio",
            Promise::Tape => "tape",
            Promise::Tmppath => "tmppath",
            Promise::Tty => "tty",
            Promise::Unix => "unix",
            Promise::Unveil => "unveil",
            Promise::Video => "video",
            Promise::Vminfo => "vminfo",
            Promise::Vmm => "vmm",
            Promise::Wpath => "wpath",
            Promise::Wroute => "wroute",
            _ => panic!("Promise::to_promise_string is incomplete (bug)"),
        }
    }
}
