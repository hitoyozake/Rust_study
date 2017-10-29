fn main() {
    println!("Hello, world!");

    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let whole = &a[..];
    let part = &a[0..4];
    let part_2 = &a[5..];

    println!("whole : ");
    for i in whole {
        println!("{}", i);
    }
    println!("part:");
    for i in part {
        println!("{}", i);
    }

    println!("part2:");
    for i in part_2 {
        println!("{}", i);
    }

}
