extern crate hyper;
extern crate tokio_core;

fn main() {
    use hyper::Client;
    use tokio_core::reactor::Core;
    let URL = "http://http2bin.org.get";
    let core = Core::new();

    let client = Client::new();

    let res = client.get(URL).send().unwrap();

    match res {
        hyper::Ok => println!("ok"),
        _ => println!("failed"),
    }
    
    println!("Hello, world!");
}
