trait Tweet {
    fn tweet(&self);

    fn tweet_twice(&self){
        self.tweet();
        self.tweet();
    }

    fn shout(&self){
        println!("SHOUT!!");
    }
}

struct Dove;
struct Duck;

impl Tweet for Dove{
    fn tweet(&self){
        println!("Coo!");
    }
}

impl Tweet for Duck{
    fn tweet(&self){
        println!("Quack!");
    }
}



fn main(){
    let arr: [i32; 3] = [0, 1, 2];
    
    println!("{:?}", &arr[0..3]);

    let dove = Dove{};
    let duck = Duck{};

    dove.tweet();
    duck.tweet();

    dove.tweet_twice();
    duck.tweet_twice();

}