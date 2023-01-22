fn main() {
    println!("LifetimeCheck");

    let mut x = 5; // x life time starts

    let y = &x; // y life time starts

    let mut z = &mut x; // z life time starts 

    //*z += 2;
    //println!("{}", z); // z life time ends()
    println!("{}", x); // y life time ends
    // x life time ends

}
