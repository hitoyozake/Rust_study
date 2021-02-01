async fn add(left: i32, right: i32)->i32{
    left + right
}

use async_trait::async_trait;

#[async_trait]
trait AsyncTrait{
    async fn f(){
        println!("ok");
    }
}


#[async_std::main]
async fn main(){
    let ans = add(2, 3).await;
    println!("{}", ans); 
}


/*
#[tokio::main]
async fn main() {
    let ans = add(2, 3).await;
    println!("{}", ans);
}
*/
