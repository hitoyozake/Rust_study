use std::thread;

fn main(){
    let mut handles = Vec::new();

    for x in 0..10{
        handles.push(thread::spawn(move||{
            println!("Hello, world!; {}", x);
        }));
    }

        for handle in handles{
            let _ = handle.join();
        }

    let hnd = thread::spawn( ||{
        println!("thread");
    });

    dbg!(hnd.join());
}