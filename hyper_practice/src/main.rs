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
    let URL = "http://wa3.i-3-i.info/word11871.html".parse().unwrap();


    let mut core = Core::new().unwrap();

    let client = Client::new(&core.handle());

    let work = client.get(URL).and_then(|res|{
        println!("Response: {}", res.status());
        println!("Headers: \n{}", res.headers());

        res.body().for_each(|chunk|{
            io::stdout().write_all(&chunk).map_err(From::from)
        })
    }).map(|_|{
        println!("\n\nDone");
    });

    core.run(work).unwrap();
    
}
