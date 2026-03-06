fn bubble_sort(arr: &mut [i32; 8]) {
    let len = arr.len();

    for i in 0..len {
        // passes
        for j in 0..len - 1 - i {
            // comparisons per pass
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
            // println!("{:?}", arr);
        }
    }
}

fn main() {
    let mut numbers = [64, 34, 25, 12, 22, 11, 90, 45];

    println!("Before sort : {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("After sort  : {:?}", numbers);
}
