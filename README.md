# pledge-rs

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/pledge)](https://crates.io/crates/pledge)

A Rust binding to OpenBSD's pledge(2) interface.

## Usage

    #[macro_use] extern crate pledge;
    use pledge::{pledge, Promise, ToPromiseString};

    fn foo() {
        // make both promises and execpromises
        match pledge![Stdio Proc Exec, Stdio Tty] {
            Err(_) => println!("Failed to pledge"),
            _ => ()
        }

        // make promises only
        match pledge_promises[Stdio Exec] {
            Err(_) => println!("Failed to pledge"),
            _ => ()
        }

        // make execpromises only
        match pledge_execpromises[Stdio] {
            Err(_) => println!("Failed to pledge"),
            _ => ()
        }
    }

This is equivalent to:

    extern crate pledge;
    use pledge::{pledge, Promise, ToPromiseString};

    fn foo() {
        // make both promises and execpromises
        let promises = vec![Promise::Stdio, Promise::Proc, Promise::Exec];
        let execpromises = vec![Promise::Stdio, Promise::Tty];
        match pledge(&*promises.to_promise_string(), &*execpromises.to_promise_string()) {
            Err(_) => println!("Failed to pledge"),
            _ => ()
        }

        // make promises only
        let promises = vec![Promise::Stdio, Promise::Exec];
        match pledge(&*promises.to_promise_string(), None) {
            Err(_) => println!("Failed to pledge"),
            _ => ()
        }

        // make execpromises only
        let execpromises = vec![Promise::Stdio];
        match pledge(None, &*execpromises.to_promise_string()) {
            Err(_) => println!("Failed to pledge"),
            _ => ()
        }
    }

You may also provide promises directly as a string:

    use pledge::pledge;

    fn foo() {
        // make both promises and execpromises
        if pledge("stdio proc exec", "stdio tty").is_err() {
            panic!("Failed to pledge");
        }

        // make promises only
        if pledge("stdio exec", None).is_err() {
            panic!("Failed to pledge");
        }

        // make execpromises only
        if pledge(None, "stdio").is_err() {
            panic!("Failed to pledge");
        }
    }

## Compatibility

This version of the crate is compatible with the [OpenBSD 6.3+ interface], where
the second parameter restricts the privileges of the process after execve(2),
and guaranteed to be compatible with Rust 1.24.0+ (as shipped by OpenBSD 6.3).

Use version `^0.3` for the [OpenBSD 5.9+ interface] last supported by [Bitrig],
where the second parameter sets a whitelist of permitted paths.

To migrate your code from older versions:

* change `pledge![P, Q, R]` call sites to `pledge_promises![P, Q, R]`
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
