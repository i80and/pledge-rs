# pledge-rs

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/hyper)](https://crates.io/crates/hyper)

A Rust binding to OpenBSD's pledge(2) interface.

## Usage

    #[macro_use] extern crate pledge;

    fn foo() {
        match pledge![Stdio, RPath] {
            Err(_) => println!("Failed to pledge"),
            _ => ()
        }
    }

This is equivalent to:

    extern crate pledge;

    fn foo() {
        match pledge::pledge(&vec![Stdio, RPath]) {
            Err(_) => println!("Failed to pledge"),
            _ => ()
        }
    }
