# pledge-rs

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/pledge)](https://crates.io/crates/pledge)

A Rust binding to OpenBSD's pledge(2) interface.

## Usage

    /* Rust 2015 only */ #[macro_use] extern crate pledge;
    /* Rust 2018 only */ use pledge::{pledge, pledge_promises, pledge_execpromises};

    fn foo() {
        // make both promises and execpromises
        pledge![Stdio Proc Exec, Stdio Tty].unwrap();

        // make promises only
        pledge_promises![Stdio Exec].unwrap();

        // make execpromises only
        pledge_execpromises![Stdio].unwrap();
    }

This is roughly equivalent to:

    /* Rust 2015 only */ extern crate pledge;
    use pledge::{pledge, Promise, ToPromiseString};

    fn foo() {
        // make both promises and execpromises
        let promises = vec![Promise::Stdio, Promise::Proc, Promise::Exec];
        let execpromises = vec![Promise::Stdio, Promise::Tty];
        pledge(&*promises.to_promise_string(), &*execpromises.to_promise_string()).unwrap();

        // make promises only
        let promises = vec![Promise::Stdio, Promise::Exec];
        pledge(&*promises.to_promise_string(), None).unwrap();

        // make execpromises only
        let execpromises = vec![Promise::Stdio];
        pledge(None, &*execpromises.to_promise_string()).unwrap();
    }

You may also provide promises directly as a string:

    /* Rust 2015 only */ extern crate pledge;
    use pledge::pledge;

    fn foo() {
        // make both promises and execpromises
        pledge("stdio proc exec", "stdio tty").unwrap();

        // make promises only
        pledge("stdio exec", None).unwrap();

        // make execpromises only
        pledge(None, "stdio").unwrap();
    }

All of these will fail on platforms other than OpenBSD. You can use conditional
compilation to make your program portable to other platforms:

    /* Rust 2015 only */ extern crate pledge;
    /* Rust 2018 only */ use pledge::pledge_promises;

    fn foo() {
        ...

        #[cfg(target_os = "openbsd")]
        pledge_promises![Stdio Exec];

        ...
    }

## Compatibility

This version of the crate is compatible with the [OpenBSD 6.3+ interface], where
the second parameter restricts the privileges of the process after execve(2),
and guaranteed to be compatible with Rust 1.24.0+ (as shipped by OpenBSD 6.3).

Use version `^0.3` for the [OpenBSD 5.9+ interface] last supported by [Bitrig],
where the second parameter sets a whitelist of permitted paths.

To migrate your code from older versions:

* change `pledge![P, Q, R]` call sites to `pledge_promises![P Q R]`
* change `pledge("p q r")` call sites to `pledge("p q r", None)`
* change `pledge_with_paths(promises, paths)` to `pledge(promises)`
* update usage of renamed `Promise` variants (e.g. `MCast` → `Mcast`)
* consider making execpromises to restrict processes after execve(2)
* consider using [unveil(2)] and the [unveil crate] (OpenBSD 6.4+)

[OpenBSD 6.3+ interface]: https://man.openbsd.org/OpenBSD-6.3/pledge.2
[OpenBSD 5.9+ interface]: https://man.openbsd.org/OpenBSD-5.9/pledge.2
[Bitrig]: https://www.bitrig.org
[unveil(2)]: https://man.openbsd.org/OpenBSD-6.4/unveil.2
[unveil crate]: https://crates.io/crates/unveil
