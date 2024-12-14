use std::process::Command;

fn main() {
    let mut hello = Command::new("echo");
    println!(
        "{:?}",
        String::from_utf8(hello.arg("something").output().unwrap().stdout).unwrap()
    );
}
