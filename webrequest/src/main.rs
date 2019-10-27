extern crate reqwest;

use std::io::Read;

fn main() {
    println!("Hello, world!");

    let mut resp = reqwest::get("https://www.rust-lang.org").unwrap();

    let mut s = String::new();

    let result = resp.read_to_string(&mut s);

    if(result.is_ok())
    {
        println!("ok");
    }

    match result {
        Ok(_) =>
        println!("{:?}", s),
        Err(e) => {},
        
    }


}
