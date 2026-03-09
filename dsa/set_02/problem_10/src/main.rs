fn find_min_index(arr: &[i32], start: usize) -> usize {
    let mut min_idk: usize = start;

    for i in (start + 1)..arr.len() {
        // look form start+1 onwards
        if arr[i] < arr[min_idk] {
            min_idk = i;
        }
    }

    min_idk
}

fn selection_sort(arr: &mut [i32; 8]) {
    let len = arr.len();

    for i in 0..len {
        let min_idk = find_min_index(arr, i);
        if min_idk != i {
            arr.swap(i, min_idk);
        }
        println!("Pass {} : {:?}", i + 1, arr);
    }
}

fn main() {
    let mut numbers = [64, 34, 25, 12, 22, 11, 90, 45];

    println!("Before : {:?}", numbers);

    selection_sort(&mut numbers);

    println!("After  : {:?}", numbers);
}
