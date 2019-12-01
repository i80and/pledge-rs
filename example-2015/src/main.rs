#[macro_use]
extern crate pledge;

fn main() {
    pledge_promises![Stdio].unwrap();
    println!("Hello, world!");
}
