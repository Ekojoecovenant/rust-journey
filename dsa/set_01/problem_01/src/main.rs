fn find_max(arr: &[i32]) -> i32 {
    let mut max = arr[0];
    for &val in arr {
        if val > max {
            max = val;
        }
    }
    max
}
fn find_min(arr: &[i32]) -> i32 {
    let mut min = arr[0];
    for &val in arr {
        if val < min {
            min = val;
        }
    }
    min
}

fn main() {
    let numbers = [34, 7, 23, 32, 5, 62, 18, 99, 1, 45];
    println!("Array   : {:?}", numbers);
    println!("Maximum : {}", find_max(&numbers));
    println!("Minimum : {}", find_min(&numbers));
}
