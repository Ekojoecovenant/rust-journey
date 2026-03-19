use std::i32;

fn second_largest(arr: &[i32]) -> i32 {
    let mut largest = i32::MIN;
    let mut second = i32::MIN;

    for &val in arr {
        if val > largest {
            second = largest;
            largest = val;
        } else if val > second && val != largest {
            second = val;
        }
    }
    second

    /* Another solution below */
    /*let mut largest = arr[0];
    for val in arr {
        if *val > largest {
            largest = *val;
        }
    }

    let mut second_largest = if arr[0] < largest { arr[0] } else { arr[1] };
    for val in arr {
        if *val < largest && *val > second_largest {
            second_largest = *val;
        }
    }

    second_largest
    */
}

fn main() {
    let numbers = [34, 7, 23, 32, 5, 62, 18, 99, 1, 45];

    println!("Array          : {:?}", numbers);
    println!("Second Largest : {}", second_largest(&numbers));
}
