fn count_occurrences(arr: &[i32], target: i32) -> u32 {
    let mut count = 0;
    for arr_val in arr {
        if *arr_val == target {
            count += 1;
        }
    }
    count
}

fn main() {
    let numbers = [1, 3, 7, 3, 2, 3, 8, 3, 5, 3];
    let target = 3;
    println!("Array  : {:?}", numbers);
    println!("Target : {}", target);
    println!("Count  : {}", count_occurrences(&numbers, target));
}
