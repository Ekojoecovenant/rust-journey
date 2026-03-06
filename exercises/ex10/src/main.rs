fn greet(name: &str, age: u32) {
    println!("Hello! My name is {name} and I am {age} years old.");
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (((c * 9.0 / 5.0) + 32.0) * 100.0).round() / 100.0
}

fn main() {
    println!("=== Exercise 10 ===");
    greet("Cove", 19);
    println!("Is 42 even? {}", is_even(42));
    println!("Is 7 even? {}", is_even(7));
    println!("37°C = {}°F", celsius_to_fahrenheit(37.0));
    println!("100°C = {}°F", celsius_to_fahrenheit(100.0));
}
