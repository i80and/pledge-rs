use pledge::pledge_promises;

fn main() {
    pledge_promises![Stdio].unwrap();
    println!("Hello, world!");
}
