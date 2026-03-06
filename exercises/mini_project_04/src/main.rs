fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn substract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return 0.0;
    }

    let val = (a / b * 100.0).round() / 100.0;
    val
}

fn power(base: f64, exp: u32) -> f64 {
    let mut result: f64 = 1.0;

    for _ in 0..exp {
        result *= base;
    }
    result
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn absolute(n: f64) -> f64 {
    (n * n).sqrt()
}

fn div_response(div: f64) -> String {
    if div == 0.0 {
        String::from("Cannot divide!")
    } else {
        div.to_string()
    }
}

fn main() {
    println!("╔══════════════════════════════════╗");
    println!("║         SIMPLE CALCULATOR        ║");
    println!("╠══════════════════════════════════╣");

    println!("║  10 + 3         = {:<15}║", add(10.0, 3.0));
    println!("║  10 - 3         = {:<15}║", substract(10.0, 3.0));
    println!("║  10 * 3         = {:<15}║", multiply(10.0, 3.0));
    println!(
        "║  10 / 3         = {:<15}║",
        div_response(divide(10.0, 3.0))
    );
    println!(
        "║  10 / 0         = {:<15}║",
        div_response(divide(10.0, 0.0))
    );
    println!("║  2 ^ 10         = {:<15}║", power(2.0, 10));
    println!("║  Is 17 prime?   = {:<15}║", is_prime(17));
    println!("║  Is 18 prime?   = {:<15}║", is_prime(18));
    println!("║  |-42.5|        = {:<15}║", absolute(-42.5));

    println!("╚══════════════════════════════════╝");
}
