#[macro_use]
extern crate pledge;

fn main() {
    pledge_promises![Stdio]
        .or_else(pledge::Error::ignore_platform)
        .unwrap();
    println!("Hello, world!");
}
