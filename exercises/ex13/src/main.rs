fn main() {
    // Part 1 - Move
    println!("--- Move ---");
    let s1 = String::from("! am owned by s1");
    let s2 = s1;
    // println!("{}", s1);
    // s1 is no longer valid because it had moved the ownership of the string to s2
    println!("s2 owns: {}", s2);
    println!("[s1 is gone - moved to s2]");

    // Part 2 - Copy
    println!("\n--- Copy ---");
    let x = 42;
    let y = x;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("[both valid - i32 is Copy type - two independent values]");

    // Part 3 - Drop
    println!("\n--- Drop ---");
    {
        let inner = String::from("I will be dropped");
        println!("Inside scope: {}", inner);
    }
    // println!("{}", inner);
    // it refused to compile because the binding 'inner' is available in a different scope in the same function
    println!("[outside scope - inner is gone - memory freed]");

    // Part 4 - Clone
    println!("\n--- Clone ---");
    let s1 = String::from("original");
    let s2 = s1.clone();
    let s2 = format!("{} + cloned", s2);
    println!("s1 - {}", s1);
    println!("s2 - {}", s2);
    println!("[independent copies - modifying s2 didn't affect s1]");
}
