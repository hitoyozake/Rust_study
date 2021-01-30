use std::collections::HashMap;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}


fn main(){

    let n = read::<String>().parse::<i64>().ok().unwrap();
    let mut hashMap = HashMap::<i64, i64>::new();
    for i in 0..n{
        let x = read::<String>().parse::<i64>().ok().unwrap();
        let counter = hashMap.entry(x).or_insert(0);
        *counter += 1;
    }

    let cnt = hashMap.iter().filter(|num|{num.1%2==1});
    let mut result = 0;
    for i in cnt{
        result += 1;
    }

    println!("{}", result);



}