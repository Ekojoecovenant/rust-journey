fn rotate_left(arr: &[i32], steps: usize) -> [i32; 8] {
    let len = arr.len();
    let mut arr2: [i32; 8] = [0i32; 8];

    for i in 0..len {
        let posi = (i + len - steps) % len;
        arr2[posi] = arr[i];
    }
    arr2
}

fn rotate_right(arr: &[i32], steps: usize) -> [i32; 8] {
    let len = arr.len();
    let mut arr2: [i32; 8] = [0i32; 8];

    for i in 0..len {
        let posi = (i + steps) % len;
        arr2[posi] = arr[i];
    }
    arr2
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];

    println!("Original       : {:?}", numbers);
    println!("Rotate Left 2  : {:?}", rotate_left(&numbers, 2));
    println!("Rotate Right 2 : {:?}", rotate_right(&numbers, 2));
    println!("Rotate Left 5  : {:?}", rotate_left(&numbers, 5));
}
