
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}


fn main() {
    let s: String = read();
    let n: i32 = s.parse().ok().unwrap();

    println!("{}", n);

    for i in 0..=n{
        let s:String = if i%15 == 0 {
            "FizzBuzz".to_string()
        } else if i%3 == 0 {
            "Fizz".to_string()
        } else if i%5 == 0 {
            "Buzz".to_string()
        } else {
            i.to_string()
        };

        println!("{}", s);
    }

}
