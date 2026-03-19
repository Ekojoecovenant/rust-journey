fn binary_search(arr: &[i32], target: i32) -> i32 {
    let mut location: i32 = -1;
    let mut left: usize = 0;
    let mut right = arr.len() - 1;

    loop {
        let mid = left + (right - left) / 2;
        // print!("Left = {}, Right = {}, Mid = {}, ", left, right, mid);

        if arr[mid] < target {
            left = mid + 1;
        } else if arr[mid] > target {
            right = mid - 1;
        } else {
            location = mid as i32;
            break;
        }
        if left > right {
            break;
        }
    }

    location
}

fn generate(val: i32) -> String {
    match val {
        -1 => String::from("Not found"),
        v => format!("Found at index {}", v),
    }
}

fn main() {
    let numbers = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];

    println!("Array      : {:?}", numbers);
    println!("Search 13  : {}", generate(binary_search(&numbers, 13)));
    println!("Search 7   : {}", generate(binary_search(&numbers, 7)));
    println!("Search 4   : {}", generate(binary_search(&numbers, 4)));
    println!("Search 1   : {}", generate(binary_search(&numbers, 1)));
    println!("Search 19  : {}", generate(binary_search(&numbers, 19)));
}
