// This file was generated from the pledge(2) manual pages
// using the helper documented at </variants/README.md>.

pub enum Promise {
    /// OpenBSD 5.9–6.6
    Audio,

    /// OpenBSD 6.1–6.6
    Bpf,

    /// OpenBSD 6.0–6.6
    Chown,

    /// OpenBSD 5.9–6.6
    Cpath,

    /// OpenBSD 6.1–6.6
    Disklabel,

    /// OpenBSD 5.9–6.6
    Dns,

    /// OpenBSD 5.9–6.6
    Dpath,

    /// OpenBSD 6.1–6.6
    Drm,

    /// OpenBSD 6.3–6.6
    Error,

    /// OpenBSD 5.9–6.6
    Exec,

    /// OpenBSD 5.9–6.6
    Fattr,

    /// OpenBSD 5.9–6.6
    Flock,

    /// OpenBSD 5.9–6.6
    Getpw,

    /// OpenBSD 5.9–6.6
    Id,

    /// OpenBSD 5.9–6.6
    Inet,

    /// OpenBSD 5.9–6.0
    Ioctl,

    /// OpenBSD 6.1–6.6
    Mcast,

    /// OpenBSD 5.9–6.6
    Pf,

    /// OpenBSD 5.9–6.6
    Proc,

    /// OpenBSD 5.9–6.6
    ProtExec,

    /// OpenBSD 5.9–6.6
    Ps,

    /// OpenBSD 5.9–6.6
    Recvfd,

    /// OpenBSD 6.1–6.6
    Route,

    /// OpenBSD 5.9–6.6
    Rpath,

    /// OpenBSD 5.9–6.6
    Sendfd,

    /// OpenBSD 5.9–6.6
    Settime,

    /// OpenBSD 5.9–6.6
    Stdio,

    /// OpenBSD 6.1–6.6
    Tape,

    /// OpenBSD 5.9–6.6
    Tmppath,

    /// OpenBSD 5.9–6.6
    Tty,

    /// OpenBSD 5.9–6.6
    Unix,

    /// OpenBSD 6.4–6.6
    Unveil,

    /// OpenBSD 6.5–6.6
    Video,

    /// OpenBSD 5.9–6.6
    Vminfo,

    /// OpenBSD 6.1–6.6
    Vmm,

    /// OpenBSD 5.9–6.6
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
