use std::collections::VecDeque;




fn main() {
    let mut vecDeq = std::collections::VecDeque::new();
    vecDeq.push_front(4);
    vecDeq.push_front(2);
    let a = vecDeq.pop_back().unwrap();

    println!("{}", a);

    println!("Hello, world!");
}
