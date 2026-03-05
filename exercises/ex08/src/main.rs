fn main() {
    // Task 1: 7 times table
    let mut loop1 = 1;
    let loop1_limit = 10;

    println!("\n--- Task 1 ---");
    while loop1 <= loop1_limit {
        println!("7 x {} = {}", loop1, 7 * loop1);
        loop1 += 1;
    }

    // Task 2: Sum of all numbers from 1 to 100
    let mut sum: i32 = 0;
    for i in 1..=100 {
        sum += i;
    }
    println!("\n--- Task 2 ---");
    println!("Sum of all number from 1 to 100 is {}", sum);

    // Task 3: First number divisible by both 7 and 13 starting from 1
    let mut counter = 1;
    let value: i32 = loop {
        if counter % 7 == 0 {
            if counter % 13 == 0 {
                break counter;
            }
        }
        counter += 1;
    };
    println!("\n--- Task 3 ---");
    println!("{} is the first number divisible by 7 and 13", value);
}
