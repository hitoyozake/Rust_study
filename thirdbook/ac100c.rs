
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_splited<T: std::str::FromStr>() -> Vec<T>{

    let mut s = String::new();

    std::io::stdin().read_line(&mut s).ok();

    s.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect()

}


fn main(){
    let n = read::<String>().parse::<i64>().ok().unwrap();
    let mut v = read_splited::<i64>();

    let mut count = 0;
    for mut i in v.iter() {
        
        loop{
            let is_even = *i % 2 == 0;
            if is_even{
                *i =   *i / 2;
                count += 1;
            } else{
                break
            }   
        }
    }

    println!("{}", count);
}