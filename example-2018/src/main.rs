use pledge::pledge_promises;

fn main() {
    pledge_promises![Stdio]
        .or_else(pledge::Error::ignore_platform)
        .unwrap();
    println!("Hello, world!");
}
