fn takes_ownership(s: String) {
    println!("Function owns this: {}", s);
}

fn borrows(s: &String) -> usize {
    s.len()
}

fn gives_ownership() -> String {
    let str = String::from("I was born inside a function!");
    str
}

fn takes_age_gives_back(s: String) -> String {
    let str: String = format!("{}{}", s, " - modified!");
    str
}

fn main() {
    println!("=== Exercise 12 ===");

    println!("\n--- Takes Ownership ---");
    let s1 = String::from("You");
    takes_ownership(s1);
    println!("[after call - s1 is gone, can't use it]");

    println!("\n--- Borrowing ---");
    let original = String::from("Hello Rust");
    println!("Original: {}", original);
    println!("Length: {}", borrows(&original));
    println!("Original still valid: {}", original);

    println!("\n--- Give Ownership ---");
    println!("Received from function: {}", gives_ownership());

    println!("\n--- Takes and Gives Back ---");
    let before = String::from("Take me and bring me back");
    println!("Before: {}", before);
    let after = takes_age_gives_back(before);
    println!("After : {}", after);
}
