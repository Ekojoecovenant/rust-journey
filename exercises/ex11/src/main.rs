fn classify_score(score: u32) -> &'static str {
    match score {
        90..=100 => "A - Excellent",
        80..=89 => "B - Great",
        70..=79 => "C - Good",
        60..=69 => "D - Needs Work",
        0..=59 => "F - See me 😤",
        _ => "Invalid score",
    }
}

fn fizzbuzz(n: u32) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        String::from("FizzBuzz")
    } else if n % 3 == 0 {
        String::from("Fizz")
    } else if n % 5 == 0 {
        String::from("Buzz")
    } else {
        n.to_string()
    }
}

fn main() {
    println!("=== Exercise 11 ===");

    println!("Score 95: {}", classify_score(95));
    println!("Score 82: {}", classify_score(82));
    println!("Score 54: {}", classify_score(54));

    println!("\nFizzBuzz from 1 to 20:");
    for i in 1..=20 {
        if i != 20 {
            print!("{}, ", fizzbuzz(i))
        } else {
            print!("{}", fizzbuzz(i))
        }
    }
}
