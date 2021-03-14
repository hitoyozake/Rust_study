use std::thread;
//use std::rc::Rc; // for single threading Reference Count
use std::sync::{Arc, Mutex}; // これは use std::sync::Mutexとuse std::sync::Arcに等しい

// message passing
use std::sync::mpsc;
fn main(){
    let mut handles = Vec::new();

    let mut data = vec![1; 10];
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();

    for _ in 0..10 {
        let (snd_tx, snd_rx) = mpsc::channel();
        let (rcv_tx, rcv_rx) = mpsc::channel();

        snd_channels.push(snd_tx);
        rcv_channels.push(rcv_rx);

        handles.push(thread::spawn(move||{
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    }

    for x in 0..10{
        let _ = snd_channels[x].send(data[x]);
    }

    for x in 0..10{
        data[x] = rcv_channels[x].recv().unwrap();
    }

    for handle in handles{
        let _ = handle.join();
    }

    dbg!(data);
    


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