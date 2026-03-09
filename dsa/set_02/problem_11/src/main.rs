fn fibonacci(n: u32) -> u64 {
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

fn main() {
    println!("Fibonacci sequence (first 10):");
    for i in 0..10 {
        if i < 9 {
            print!("{}, ", fibonacci(i));
        } else {
            println!("{}", fibonacci(i));
        }
    }

    println!("fib(0)  = {}", fibonacci(0));
    println!("fib(1)  = {}", fibonacci(1));
    println!("fib(7)  = {}", fibonacci(7));
    println!("fib(10) = {}", fibonacci(10));
}
