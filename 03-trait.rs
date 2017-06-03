

fn main(){
    let c1 = Circle {x:2.0,y:2.0,radius:2.0};
    let c2 = Circle {x:2.0,y:2.0,radius:3.0};
    println!("c1 area: {}", c1.area());
    println!("c2 area: {}", c2.area());
    println!("c1 > c2: {}", c1.is_larger(&c2));
    let r1 = Rect {x:2.0,y:2.0,height:2.0, width:2.0};
    let r2 = Rect {x:2.0,y:2.0,height:2.0, width:1.0};

    println!("r1 area: {}", r1.area());
    println!("r2 area: {}", r2.area());
    println!("r1 > r2: {}", r1.is_larger(&r2));
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}
struct Rect{
    x: f64,
    y: f64,
    height: f64,
    width: f64,
}

trait Shape {
    fn area(&self) -> f64; 

    // default function
    fn is_larger(&self, other:&Self) -> bool{
        self.area() > other.area()
    }

}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius )
    }

}

impl Shape for Rect{
    fn area(&self) -> f64 {
        self.height * self.width
    }

}
