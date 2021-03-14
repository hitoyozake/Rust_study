use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    println!("Hello, world!");

    let path = Path::new("hello.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}:{}", display, Error::description(&why)),
        Ok(file) => file,
        // パターンマッチ
        //
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {} : {} ", display, Error::description(&why)),
        Ok(_) => {
            print!("{} contains:\n{}", display, s);
            let v: Vec<&str> = s.split(',').collect();
            for i in v {
                println!("{}", i);
            }
        }
    }

    // fileは自動でclose
}
