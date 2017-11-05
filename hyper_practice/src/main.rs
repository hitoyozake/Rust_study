extern crate hyper;
extern crate tokio_core;

fn main() {
    use hyper::Client;
    use tokio_core::reactor::Core;
    let URL = "http://http2bin.org.get";
    let core = Core::new();
    
    println!("Hello, world!");
}
