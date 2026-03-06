fn two_sum(arr: &[i32], target: i32) -> (i32, i32) {
    for (i1, v1) in arr.iter().enumerate() {
        for (i2, v2) in arr.iter().enumerate() {
            if i1 == i2 {
                // to make sure they dont add themselves (or should it be able to add itself?)
                continue;
            }
            if *v1 + *v2 == target {
                return (i1 as i32, i2 as i32);
            }
        }
    }

    (-1, -1)
}

// had to create a new func to create the custom response
fn generate(tup: (i32, i32), arr: &[i32]) -> String {
    if tup.0 == -1 {
        return String::from("Not found -> (-1, -1)");
    }

    format!(
        "indices {:?} -> values {} + {} = {}",
        tup,
        arr[tup.0 as usize],
        arr[tup.1 as usize],
        arr[tup.0 as usize] + arr[tup.1 as usize]
    )
}

fn main() {
    let numbers = [2, 7, 11, 15, 3, 6];

    println!("Array     : {:?}", numbers);
    println!("Target    : {}", 9);
    println!("Pair      : {}", generate(two_sum(&numbers, 9), &numbers));

    println!("\nTarget    : {}", 18);
    println!("Pair      : {}", generate(two_sum(&numbers, 18), &numbers));

    println!("\nTarget    : {}", 4);
    println!("Pair      : {}", generate(two_sum(&numbers, 4), &numbers));
}
