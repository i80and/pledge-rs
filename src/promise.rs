// This file was generated from the pledge(2) manual pages
// using the helper documented at </variants/README.md>.

pub enum Promise {
    /// OpenBSD 5.9–6.8
    Audio,

    /// OpenBSD 6.1–6.8
    Bpf,

    /// OpenBSD 6.0–6.8
    Chown,

    /// OpenBSD 5.9–6.8
    Cpath,

    /// OpenBSD 6.1–6.8
    Disklabel,

    /// OpenBSD 5.9–6.8
    Dns,

    /// OpenBSD 5.9–6.8
    Dpath,

    /// OpenBSD 6.1–6.8
    Drm,

    /// OpenBSD 6.3–6.8
    Error,

    /// OpenBSD 5.9–6.8
    Exec,

    /// OpenBSD 5.9–6.8
    Fattr,

    /// OpenBSD 5.9–6.8
    Flock,

    /// OpenBSD 5.9–6.8
    Getpw,

    /// OpenBSD 5.9–6.8
    Id,

    /// OpenBSD 5.9–6.8
    Inet,

    /// OpenBSD 5.9–6.0
    Ioctl,

    /// OpenBSD 6.1–6.8
    Mcast,

    /// OpenBSD 5.9–6.8
    Pf,

    /// OpenBSD 5.9–6.8
    Proc,

    /// OpenBSD 5.9–6.8
    ProtExec,

    /// OpenBSD 5.9–6.8
    Ps,

    /// OpenBSD 5.9–6.8
    Recvfd,

    /// OpenBSD 6.1–6.8
    Route,

    /// OpenBSD 5.9–6.8
    Rpath,

    /// OpenBSD 5.9–6.8
    Sendfd,

    /// OpenBSD 5.9–6.8
    Settime,

    /// OpenBSD 5.9–6.8
    Stdio,

    /// OpenBSD 6.1–6.8
    Tape,

    /// OpenBSD 5.9–6.8
    Tmppath,

    /// OpenBSD 5.9–6.8
    Tty,

    /// OpenBSD 5.9–6.8
    Unix,

    /// OpenBSD 6.4–6.8
    Unveil,

    /// OpenBSD 6.5–6.8
    Video,

    /// OpenBSD 5.9–6.8
    Vminfo,

    /// OpenBSD 6.1–6.8
    Vmm,

    /// OpenBSD 5.9–6.8
    Wpath,

    /// OpenBSD 6.8
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
