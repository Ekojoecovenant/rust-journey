/*fn main() {
    let numbers = &vec![1, 2, 3, 4, 5, 6];

    // let even_numbers: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    let even_numbers: Vec<i32> = numbers
        .clone()
        .iter()
        .filter(|x| *x % 2 == 0)
        .copied()
        .collect();

    println!("{:?}", numbers);
    println!("{:?}", even_numbers);
}
*/

/*fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    let multipled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

    println!("{:?}", multipled);
}
*/

/*fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let new: Vec<i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .collect();

    let sum = new.iter().fold(0, |start, &x| start + x);
    println!("{:?}", new);
    println!("{}", sum);
}
*/

fn main() {
    let bonus = 10;

    // fn add_bonus(x: i32) -> i32 {
    //     x + bonus // ❌ Rust screams here!!
    // }

    let add_bonus = |x| x + bonus;

    println!("{}", add_bonus(5));
}
