fn first_word(s: &String) -> usize {
    for (i, val) in s.chars().enumerate() {
        if val == ' ' {
            return i;
        }
    }
    s.len()
}

fn make_uppercase_length(s: &String) -> (String, usize) {
    let upper = s.to_uppercase();
    let len = upper.len();
    (upper, len)
}

fn append_excited(s: &mut String) {
    s.push_str("!!!");
}

fn main() {
    let statement = String::from("Hello, world!");

    // func 1
    println!("Function 1 : {}", first_word(&statement));

    // func 2
    let mul = make_uppercase_length(&statement);
    println!("Function 2 : String = {}, length = {}", mul.0, mul.1);

    // func 3
    let mut statement = statement;
    append_excited(&mut statement);
    println!("Function 3 : {}", statement);
}
