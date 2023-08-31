
use std::process::Command;

fn main() {

    let output = Command::new("Cargo")
        .arg("doc --no-deps --workspace --document-private-items")
        .output()
        .expect("Docs Failed to execute cargo doc command");

    // println!("Docs Hello, world!");
    // assert_eq!(b"Docs Hello world\n", output.stdout.as_slice())
}
