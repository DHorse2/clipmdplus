fn main() {
    use std::process::Command;

    let output = Command::new("Cargo")
        .arg("doc --no-deps --workspace --document-private-items")
        .output()
        .expect("Docs Failed to execute cargo doc command");

    println!("Docs {?}", output);
    // assert_eq!(b"Docs Hello world\n", output.stdout.as_slice())
}
