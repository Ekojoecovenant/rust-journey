fn two_pointer_sum(arr: &[i32], target: i32) -> Option<(i32, i32)> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let sum = arr[left] + arr[right];

        if sum == target {
            return Some((arr[left], arr[right]));
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    None
}

fn main() {
    let mut arr = [7, 11, 3, 5, 1, 9];
    let nums = [14, 4];

    arr.sort();

    for num in nums {
        println!("\nInput  : {:?}, target = {}", arr, num);
        println!("Output : {:?}", two_pointer_sum(&arr, num));
    }
}
