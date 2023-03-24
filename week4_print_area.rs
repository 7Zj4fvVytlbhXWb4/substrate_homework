trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: Shape>(shape: &T) {
    println!("The area of the shape is {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 1.0 };
    print_area(&circle);

    let triangle = Triangle { base: 1.0, height: 1.0 };
    print_area(&triangle);

    let square = Square { side: 1.0 };
    print_area(&square);
}
