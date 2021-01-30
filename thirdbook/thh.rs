
struct Point{
    x : i64,
    y : i64
}

struct Triangle{
    a: Point,  b: Point, c: Point
}

trait Area{
    fn calcArea(&self)->i64;
}

impl Area for Triangle{
    fn calcArea(&self)->i64{
        (self.a.x-self.b.x).abs() * (self.c.y-self.a.y).abs() / 2
    }
}


fn main(){
    println!("print");
    let a = Triangle{ a: Point{x:32, y:12}, b: Point{x:24, y:12},c: Point{x:24, y:16}};
    println!("{}", a.calcArea());
}