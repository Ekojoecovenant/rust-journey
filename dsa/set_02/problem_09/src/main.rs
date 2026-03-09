fn count_evens_odds(arr: &[i32]) -> (u32, u32) {
    let mut evens: u32 = 0;
    let mut odds: u32 = 0;

    for val in arr {
        match val % 2 {
            0 => evens += 1,
            _ => odds += 1,
        }
    }

    (evens, odds)
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (evens, odds) = count_evens_odds(&numbers);

    println!("Array : {:?}", numbers);
    println!("Evens : {}", evens);
    println!("Odds : {}", odds);
}
