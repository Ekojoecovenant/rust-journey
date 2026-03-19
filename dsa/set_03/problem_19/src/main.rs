fn merge_sorted(arr1: &[i32], arr2: &[i32]) -> [i32; 10] {
    let mut result = [0i32; 10];
    let mut i = 0usize;
    let mut j = 0usize;
    let mut count = 0usize;

    // it gave me  [1, 3, 5, 7, 9, 2, 4, 6, 8, 10]
    // while count < 10 {
    //     if i >= arr1.len() {
    //         result[count] = arr2[j];
    //         j += 1;
    //     } else if j >= arr2.len() {
    //         result[count] = arr1[i];
    //         i += 1;
    //     } else if arr1[i] <= arr2[i] {
    //         result[count] = arr1[i];
    //         i += 1;
    //     } else {
    //         result[count] = arr2[j];
    //         j += 1;
    //     }
    //     count += 1;
    // }

    // merge while both have elements
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result[count] = arr1[i];
            i += 1;
        } else {
            result[count] = arr2[j];
            j += 1;
        }

        count += 1;
    }

    // remaining elements from arr1
    while i < arr1.len() {
        result[count] = arr1[i];
        i += 1;
        count += 1;
    }

    // remaining elements from arr2
    while j < arr2.len() {
        result[count] = arr2[j];
        j += 1;
        count += 1;
    }

    result
}

fn main() {
    let arr1 = [1, 3, 5, 7, 9];
    let arr2 = [2, 4, 6, 8, 10];

    println!("Array 1 : {:?}", arr1);
    println!("Array 2 : {:?}", arr2);
    println!("Merged  : {:?}", merge_sorted(&arr1, &arr2));
}
