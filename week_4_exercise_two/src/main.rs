struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * 3.142
    }
    fn circumference(&self) -> f64 {
        2.0 * 3.142 * self.radius
    }
}

fn main() {
    let circ: Circle = Circle { radius: 5.0 };
    println!("Area: {}", circ.area());
    println!("Circumference: {}", circ.circumference());
}
