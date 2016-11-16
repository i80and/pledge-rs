# pledge-rs

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/pledge)](https://crates.io/crates/pledge)

A Rust binding to OpenBSD's pledge(2) interface.

## Usage

    #[macro_use] extern crate pledge;
    use pledge::{pledge, Promise};

    fn foo() {
        match pledge![Stdio, RPath] {
            Err(_) => println!("Failed to pledge"),
            _ => ()
        }
    }

This is equivalent to:

    extern crate pledge;
    use pledge::{pledge, Promise, ToPromiseString};

    fn foo() {
        match pledge(&vec![Promise::Stdio, Promise::RPath].to_promise_string()) {
            Err(_) => println!("Failed to pledge"),
            _ => ()
        }
    }

You may also provide promises directly as a string:

    use pledge::pledge;

    fn foo() {
        if pledge("stdio rpath").is_err() {
            panic!("Failed to pledge");
        }
    }
