// This file was generated from the pledge(2) manual pages
// using the helper documented at </variants/README.md>.

pub enum Promise {
    /// OpenBSD 5.9–6.7
    Audio,

    /// OpenBSD 6.1–6.7
    Bpf,

    /// OpenBSD 6.0–6.7
    Chown,

    /// OpenBSD 5.9–6.7
    Cpath,

    /// OpenBSD 6.1–6.7
    Disklabel,

    /// OpenBSD 5.9–6.7
    Dns,

    /// OpenBSD 5.9–6.7
    Dpath,

    /// OpenBSD 6.1–6.7
    Drm,

    /// OpenBSD 6.3–6.7
    Error,

    /// OpenBSD 5.9–6.7
    Exec,

    /// OpenBSD 5.9–6.7
    Fattr,

    /// OpenBSD 5.9–6.7
    Flock,

    /// OpenBSD 5.9–6.7
    Getpw,

    /// OpenBSD 5.9–6.7
    Id,

    /// OpenBSD 5.9–6.7
    Inet,

    /// OpenBSD 5.9–6.0
    Ioctl,

    /// OpenBSD 6.1–6.7
    Mcast,

    /// OpenBSD 5.9–6.7
    Pf,

    /// OpenBSD 5.9–6.7
    Proc,

    /// OpenBSD 5.9–6.7
    ProtExec,

    /// OpenBSD 5.9–6.7
    Ps,

    /// OpenBSD 5.9–6.7
    Recvfd,

    /// OpenBSD 6.1–6.7
    Route,

    /// OpenBSD 5.9–6.7
    Rpath,

    /// OpenBSD 5.9–6.7
    Sendfd,

    /// OpenBSD 5.9–6.7
    Settime,

    /// OpenBSD 5.9–6.7
    Stdio,

    /// OpenBSD 6.1–6.7
    Tape,

    /// OpenBSD 5.9–6.7
    Tmppath,

    /// OpenBSD 5.9–6.7
    Tty,

    /// OpenBSD 5.9–6.7
    Unix,

    /// OpenBSD 6.4–6.7
    Unveil,

    /// OpenBSD 6.5–6.7
    Video,

    /// OpenBSD 5.9–6.7
    Vminfo,

    /// OpenBSD 6.1–6.7
    Vmm,

    /// OpenBSD 5.9–6.7
    Wpath,

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
            _ => panic!("Promise::to_promise_string is incomplete (bug)"),
        }
    }
}
