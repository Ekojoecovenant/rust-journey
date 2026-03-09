fn insertion_sort(arr: &mut [i32; 8]) {
    let len = arr.len();

    for i in 1..len {
        let key: i32 = arr[i]; // element to be inserted
        let mut j = i;

        while j > 0 && arr[j - 1] > key {
            // while left neighbor is bigger
            arr[j] = arr[j - 1]; // shift left neightbor RIGHT
            j -= 1; // move j left
        }

        arr[j] = key; // insert key at correct position
        println!("After insert : {:?}", arr);
    }
}

fn main() {
    let mut numbers = [64, 34, 25, 12, 22, 11, 90, 45];

    println!("Before       : {:?}", numbers);

    insertion_sort(&mut numbers);

    println!("After        : {:?}", numbers);
}
