use std::thread;
//use std::rc::Rc; // for single threading Reference Count
use std::sync::{Arc, Mutex}; // これは use std::sync::Mutexとuse std::sync::Arcに等しい

// message passing
use std::sync::mpsc;
fn main(){
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = rx.recv().unwrap();
        println!("{}", data);
    });

    let _ = tx.send("Hello, world!");

    let _ = handle.join();
}


// 共有メモリ
fn main3(){
    let mut handles = Vec::new();

    let mut data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10{
        let data_ref = data.clone();
        handles.push(thread::spawn(move||{
            let mut d = data_ref.lock().unwrap();
            d[x] += 1;
        }))
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);

}



fn main2(){
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