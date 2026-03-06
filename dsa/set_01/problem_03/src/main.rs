fn reverse_array(arr: &[i32]) -> [i32; 10] {
    let mut result = [0i32; 10];

    for (index, val) in arr.iter().enumerate() {
        result[10 - index - 1] = *val;
    }

    result
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("Original : {:?}", numbers);
    println!("Reversed : {:?}", reverse_array(&numbers));
}
