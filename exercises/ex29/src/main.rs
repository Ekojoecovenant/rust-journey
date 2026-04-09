use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;

    fn name(&self) -> String;

    fn describe(&self) {
        println!("Shape: {}\nArea: {:.2}", self.name(), self.area());
    }
}

struct Circle {
    radius: f64,
}
struct Rectangle {
    width: f64,
    height: f64,
}
struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn name(&self) -> String {
        "Circle".to_string()
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn name(&self) -> String {
        "Rectangle".to_string()
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    fn name(&self) -> String {
        "Triangle".to_string()
    }
}

fn print_shape(shape: &impl Shape) {
    shape.describe();
}

fn largest_shape<T: Shape, U: Shape>(a: &T, b: &U) -> String {
    let area_of_a = a.area();
    let area_of_b = b.area();

    if area_of_a == area_of_b {
        String::from("Equal")
    } else if area_of_a > area_of_b {
        a.name()
    } else {
        b.name()
    }
}

fn main() {
    let circle = Circle { radius: 10.0 };
    let rectangle = Rectangle {
        width: 7.0,
        height: 8.0,
    };
    let triangle = Triangle {
        base: 5.0,
        height: 9.0,
    };

    print_shape(&circle);
    println!("");
    print_shape(&rectangle);
    println!("");
    print_shape(&triangle);

    println!("\n{}", largest_shape(&circle, &triangle));
    println!("\n{}", largest_shape(&rectangle, &triangle));
}
