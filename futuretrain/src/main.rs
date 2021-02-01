use futures::{executor, future::join_all};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CountDown(u32);

impl Future for CountDown{
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<String>{
        if self.0 == 0{
            Poll::Ready("Zero!!".to_string())
        } else{
            println!("{}", self.0);
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn mainx(){

    let countdown_future1 = CountDown(10);
    let countdown_future2 = CountDown(20);
    
    let cd_set = join_all(vec![countdown_future1, countdown_future2]);
    let res = executor::block_on(cd_set);
    for(i , s) in res.iter().enumerate(){
        println!("{}: {}", i, s);
    }
}

async fn async_add(a: i32, b: i32)->i32{
    a+b
}


async fn something_great_async_function()->i32{
    let ans = async_add(2, 3).await;

    println!("{}", ans);
    ans
}

fn something_gread_async_function2()->impl Future<Output = i32>{
    async {
        let ans = async_add(2, 3).await; // awaitはasyncの中でのみ使用可能

        println!("{}", ans);
        ans
    }
}

fn main(){
    executor::block_on(something_great_async_function());   
    executor::block_on(something_gread_async_function2());
}
