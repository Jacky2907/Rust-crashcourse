use std::ops::Mul;
struct Triangle{
    val_base : f64,
    val_h : f64,
}

impl Triangle{  //Constructeur
    pub fn new(b : f64, h : f64) -> Triangle{
        Triangle{val_base : b, val_h : h}
    }
}

struct Square <T>{
    cote : T
}

impl<T> Square<T> {
    pub fn new(c : T) -> Square<T> {
        Square{cote : c}
    }

    // pub fn area(& self) -> &T as Mul<&>>::Output {
    //     self.cote*self.cote
    // }
}

/* struct Pyramid <T>{
    cote : T
}


impl<T> Pyramid<T> {
    pub fn new(c : T) -> Pyramid<T> {
        Pyramid{cote : c}
    }
}*/
fn main() {
    let square = Square::<u32>::new(5);
    let square_float = Square::<f64>::new(5.4);
    let square_string = Square::<String>::new("6".to_string());

    println!("square area is {}", square.area());
    println!("square_float area is {}", square_float.area());
    println!("square_string area is {}", square_string.area());

    let triangle = Triangle::new(14.9, 20.1);
    let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    let pyramid_triangle = Pyramid::<Triangle<f64>, f64>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
}
