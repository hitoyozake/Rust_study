use std::process::Command;

fn main() {
     let output = Command::new("ls")
     .output().expect("failed to run command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("Hello, world!");
}
