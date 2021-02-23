use std::process::Command;
use std::env;
fn main() {
     let output = Command::new("ls")
     .output().expect("failed to run command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("Hello, world!");

    let args:Vec<String> = env::args().collect();

    let infilename = &args[1];

    println!("{}", infilename);
}
