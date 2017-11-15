extern crate hyper;
extern crate tokio_core;
extern crate futures;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

fn main() {
    use hyper::Client;
    use tokio_core::reactor::Core;
    let URL = "http://http2bin.org.get".parse().unwrap();


    let mut core = Core::new().unwrap();

    let client = Client::new(&core.handle());

    let work = client.get(URL);
    
    println!("Hello, world!");
}
