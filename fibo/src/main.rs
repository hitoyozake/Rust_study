fn fibo(n: i32)->i32{

    match(n){
        0|1 => 1,
        _ => fibo(n-1) + fibo(n-2)
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn fibotest(){
    assert_eq!(fibo(0),1);
    assert_eq!(fibo(1),1);
}


#[test]
fn fibotest2(){
    assert_eq!(fibo(7),21);
}