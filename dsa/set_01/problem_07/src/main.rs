// fn is_palindrome(s: &str) -> bool {
//     let mut s = s.to_lowercase();
//     let str_len = s.len();

//     for _ in 0..str_len {
//         if s.len() == 1 {
//             return true;
//         }

//         let f = s.remove(0);
//         let l = s.remove(s.len() - 1);

//         if f != l {
//             return false;
//         }
//     }
//     println!("{}", s);

//     true
// }

fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.to_lowercase().chars().collect();
    let len = chars.len();

    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

fn main() {
    println!("\"racecar\" is palindrome: {}", is_palindrome("racecar"));
    println!("\"hello\"   is palindrome: {}", is_palindrome("hello"));
    println!("\"Madam\"   is palindrome: {}", is_palindrome("Madam"));
    println!("\"Rust\"    is palindrome: {}", is_palindrome("Rust"));
    println!("\"level\"   is palindrome: {}", is_palindrome("level"));
}
