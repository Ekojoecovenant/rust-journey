fn sum(arr: &[i32]) -> i32 {
    let mut total = 0;
    for i in arr {
        total += i;
    }
    total
}

fn average(arr: &[i32]) -> f64 {
    let avg = sum(&arr) as f64 / arr.len() as f64;
    let avg = (avg * 100.0).round() / 100.0;
    avg
}

fn main() {
    let number = [12, 45, 7, 23, 56, 89, 34, 67, 11, 43];

    println!("Array   : {:?}", number);
    println!("Sum     : {}", sum(&number));
    println!("Average : {}", average(&number));
}
