use std::thread;

fn main(){
    let hnd = thread::spawn(||{
        println!("thread");
    });

    dbg!(hnd.join());
}