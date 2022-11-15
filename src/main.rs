trait Area {
    fn area(&self) -> f64;
}
trait Volume {
    fn volume(&self) -> f64;
}
struct Square<New> 
{
    s: New, //S means side of the square
}

struct Triangle<T=f64> {
    base: T,
    height: T
}
struct Pyramid<T, U> {
    base : T,
    height : U
}

impl Square<f64> {
    fn new( t: f64) -> Self {
        Square { s : t}
    }
}
impl Area for Square<f64> {
    fn area(&self) -> f64 {
        self.s * self.s
    }
}
impl Square<u32> {
    fn new( t: u32) -> Self {
        Square { s : t}
    }
}

impl Area for Square<u32> {
    fn area(&self) -> f64 {
        (self.s * self.s).into()
    }
}


impl Square<String> {
    fn new( t: &str) -> Self {
        let float : f64;
        float = t.parse().unwrap();
        Square { s : (float).to_string(),
    }
}
}

impl Area for Square<String> {
    fn area(&self) -> f64 {
        let tamp : f64 = self.s.parse().unwrap();
        tamp*tamp
    }
}

impl Triangle<f64> {
    fn new( b: f64, h: f64) -> Self {
        Triangle {base: b, height: h} 
    }
}

impl Area for Triangle<f64> {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}

impl Pyramid<Square<u32>, f64> {
    fn new(new_base : Square<u32>, new_height : f64) -> Self {
        Pyramid { base : new_base, height : new_height}
    }
}
impl Volume for Pyramid<Square<u32>, f64> {
    fn volume(&self) -> f64 {
        self.base.area() * self.height / 3.0
    }
}
impl Pyramid<Triangle<f64>, f64> {
    fn new(new_base : Triangle<f64>, new_height : f64) -> Self {
        Pyramid { base : new_base, height : new_height}
    }
}

impl Volume for Pyramid<Triangle<f64>, f64> {
    fn volume(&self) -> f64 {
        self.base.area() * self.height / 3.0
    }
}





fn main() {
    let square = Square::<u32>::new(5);
    let square_float = Square::<f64>::new(5.4);
    let square_string = Square::<String>::new("6");

    println!("square area is {}", square.area());
    println!("square_float area is {}", square_float.area());
    println!("square_string area is {}", square_string.area());

    let triangle = Triangle::new(14.9, 20.1);
    let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    let pyramid_triangle = Pyramid::<Triangle<f64>, f64>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
}