use std::{f64::consts::PI, fmt};

trait Shape {
    fn name(&self) -> String;

    fn area(&self) -> f64;

    fn perimeter(&self) -> f64;

    fn describe(&self) {
        println!("=== {} ===", self.name());
        println!("Area      : {:.2}", self.area());
        println!("Perimeter : {:.2}", self.perimeter());
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
    a: f64,
    b: f64,
    c: f64,
}

struct Square {
    side: f64,
}

struct Trapezoid {
    a: f64,
    b: f64,
    height: f64,
}

impl Shape for Circle {
    fn name(&self) -> String {
        String::from("Circle")
    }

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

impl Shape for Rectangle {
    fn name(&self) -> String {
        String::from("Rectangle")
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Shape for Triangle {
    fn name(&self) -> String {
        String::from("Triangle")
    }

    fn area(&self) -> f64 {
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

impl Shape for Square {
    fn name(&self) -> String {
        String::from("Square")
    }

    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
}

impl Shape for Trapezoid {
    fn name(&self) -> String {
        String::from("Trapezoid")
    }

    fn area(&self) -> f64 {
        0.5 * (self.a + self.b) * self.height
    }

    fn perimeter(&self) -> f64 {
        // Calculate the base of the small side triangle
        let side_base = (self.a - self.b).abs() / 2.0;
        // Use Pythagoras to find the length of the slanted leg
        let leg = (side_base.powi(2) + self.height.powi(2)).sqrt();

        self.a + self.b + (2.0 * leg)
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "[{}] Area: {:.2}, Perimeter : {:.2}",
                self.name(),
                self.area(),
                self.perimeter()
            ),
        )
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "[{}] Area: {:.2}, Perimeter : {:.2}",
                self.name(),
                self.area(),
                self.perimeter()
            ),
        )
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "[{}] Area: {:.2}, Perimeter : {:.2}",
                self.name(),
                self.area(),
                self.perimeter()
            ),
        )
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "[{}] Area: {:.2}, Perimeter : {:.2}",
                self.name(),
                self.area(),
                self.perimeter()
            ),
        )
    }
}

impl fmt::Display for Trapezoid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] Area: {:.2}, Perimeter : {:.2}",
            self.name(),
            self.area(),
            self.perimeter()
        )
    }
}

fn largest_area(shapes: &[&dyn Shape]) -> String {
    let mut largest = (0.0, String::from(""));

    for shape in shapes {
        if shape.area() > largest.0 {
            largest.0 = shape.area();
            largest.1 = shape.name();
        }
    }

    largest.1
}

fn total_perimeter(shapes: &[&dyn Shape]) -> f64 {
    let mut perimeter = 0.0;

    for shape in shapes {
        perimeter += shape.perimeter();
    }
    perimeter
}

fn filter_by_area(shapes: &[&dyn Shape], min: f64) -> Vec<String> {
    let mut passed = Vec::new();

    for shape in shapes {
        if shape.area() >= min {
            passed.push(shape.name());
        }
    }

    passed
}

fn main() {
    let circle: Circle = Circle { radius: 9.0 };

    let rectangle: Rectangle = Rectangle {
        width: 4.5,
        height: 6.0,
    };

    let triangle: Triangle = Triangle {
        a: 3.0,
        b: 4.0,
        c: 5.0,
    };

    let square: Square = Square { side: 7.0 };

    let trapezoid: Trapezoid = Trapezoid {
        a: 4.0,
        b: 2.0,
        height: 5.5,
    };

    circle.describe();
    println!("");
    rectangle.describe();
    println!("");
    triangle.describe();
    println!("");
    square.describe();
    println!("");
    trapezoid.describe();

    let shapes: Vec<&dyn Shape> = vec![&circle, &rectangle, &triangle, &square, &trapezoid];

    println!("\nLargest area    : {}", largest_area(&shapes));
    println!("Total Perimeter : {:.2}", total_perimeter(&shapes));
    println!("Filter by area  : {:?}", filter_by_area(&shapes, 30.0));
}
