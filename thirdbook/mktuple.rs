
// genericsの練習
fn make_tuple<T, S>(t: T, s: S)->(T, S){
    (t, s)
}

fn main(){

    let t1 = make_tuple(1, 2);
    let t2 = make_tuple("Hello", "World!");
    let t3 = make_tuple("Hoge", 1);
}