fn fibonacci_recursive(n: u32) -> u64 {
    if n == 0 || n == 1 {
        return n as u64;
    }

    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

// tbh..i just copied this code from problem_11
fn fibonacci_iterative(n: u32) -> u64 {
    if n == 0 || n == 1 {
        return n as u64;
    }

    let mut fib: (u64, u64) = (0, 1);
    let mut count: u32 = 1;
    loop {
        count += 1;

        if count > n {
            return fib.1;
        }

        (fib.0, fib.1) = (fib.1, fib.0 + fib.1);
    }
}

fn is_match(r: u64, i: u64) -> &'static str {
    if r == i { "✅" } else { "❌" }
}

fn main() {
    println!("=== Fibonacci Comparison ===");
    println!("n   Recursive  Iterative  Match?");

    for n in 0u32..=10 {
        let fibr = fibonacci_recursive(n);
        let fibi = fibonacci_iterative(n);
        println!(
            "{:<3} {:<10} {:<10} {}",
            n,
            fibr,
            fibi,
            is_match(fibr, fibi)
        );
    }
    // recursive is O(2^n)
}
