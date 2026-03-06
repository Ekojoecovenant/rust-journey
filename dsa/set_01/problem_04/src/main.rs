fn linear_search(arr: &[i32], target: i32) -> i32 {
    for (index, value) in arr.iter().enumerate() {
        if target == *value {
            return index as i32;
        }
    }
    -1
}

fn generate(target: i32) -> String {
    if target < 0 {
        String::from("Not Found")
    } else {
        format!("Found at index {}", target)
    }
}

fn main() {
    let numbers = [10, 25, 37, 42, 58, 71, 83, 96];

    println!("Array     : {:?}", numbers);
    println!("Search 42 : {}", generate(linear_search(&numbers, 42)));
    println!("Search 50 : {}", generate(linear_search(&numbers, 50)));
}
