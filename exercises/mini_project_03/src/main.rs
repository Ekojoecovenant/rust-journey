// Number Analyzer
fn main() {
    let number: i32 = 42;

    println!("=== Number Analysis: {} ===", number);

    // Type
    let num_type: &str = if number % 2 == 0 { "Even" } else { "Odd" };
    println!("Type      : {}", num_type);

    // Size
    let num_size: &str = match number {
        1..=10 => "Tiny (1-10)",
        11..=100 => "Medium (11-100)",
        101..=1000 => "Large (101-1000)",
        _ => "Huge (1000+)",
    };
    println!("Size      : {}", num_size);

    // FizzBuzz
    let num_fuzz_buzz: &str = if number % 3 == 0 && number % 5 == 0 {
        "FizzBuzz (÷3 and ÷5)"
    } else if number % 3 == 0 {
        "Fizz (÷3)"
    } else if number % 5 == 0 {
        "Buzz (÷5)"
    } else {
        "Neither"
    };
    println!("FuzzBuzz  : {}", num_fuzz_buzz);

    // Divisors
    let mut count = 0;
    print!("Divisors  : ");
    for i in 1..=number {
        if number % i == 0 {
            print!("{} ", i);
            count += 1;
        }
    }
    println!("");

    // Count
    println!("Count     : {} divisors", count);

    // Square
    println!("Square    : {}", number.pow(2));

    // Sqrt
    let sqrt = (number as f64).sqrt();
    let sqrt = (sqrt * 100.0).round() / 100.0;
    println!("Sqrt      : {}", sqrt);

    // Fun Fact
    let fun_fact = if number == 42 {
        "The Answer to Life, Universe & Everything!"
    } else {
        "No fun fact"
    };
    println!("Fun Fact  : {}", fun_fact);

    // let mut numDivisors: &str;
}
