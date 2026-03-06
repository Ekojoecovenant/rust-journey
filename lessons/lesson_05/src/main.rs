// fn say_hello() {
//     println!("Hello from a function!");
// }

// fn main() {
//     say_hello(); // calling the function
//     say_hello(); // calling it again — reusable!
//     say_hello(); // and again!
// }

// Function with parameters
// fn greet(name: &str) {
//     println!("Hello, {}! Welcome to Rust!", name);
// }

// fn main() {
//     greet("Cove");
//     greet("Rustacean");
// }

// Multiple parameters
// fn describe_person(name: &str, age: u32, is_student: bool) {
//     println!("Name: {}", name);
//     println!("Age: {}", age);
//     println!("Student: {}", is_student);
// }

// fn main() {
//     describe_person("Cove", 19, true);
//     describe_person("Ade", 25, false);
// }

// Functions that return values
// fn add(a: i32, b: i32) -> i32 {
//     a + b // no semicolon - this is the return value
// }

// fn main() {
//     let result = add(5, 3);
//     println!("5 + 3 = {}", result);

//     println!("10 + 20 = {}", add(10, 20));
// }

// Early return with "return" keyword
// fn check_age(age: u32) -> &'static str {
//     if age < 18 {
//         return "Too young!"; // exit immediately
//     }

//     if age > 120 {
//         return "Invalid age!"; // exit immediately
//     }

//     "Valid age!" // normal return at the end
// }

// fn main() {
//     println!("{}", check_age(15));
//     println!("{}", check_age(200));
//     println!("{}", check_age(25));
// }

// Functions calling other functions
// fn square(n: i32) -> i32 {
//     n * n
// }

// fn sum_of_sqares(a: i32, b: i32) -> i32 {
//     square(a) + square(b)
// }

// fn main() {
//     let result = sum_of_sqares(3, 4);
//     println!("Sum of squares: {}", result);
// }

// Statement vs Expressions
// fn main() {
//     // This block is an expression — produces 6
//     let result = {
//         let y = 3;
//         y * 2 // no semicolon — this is the block's value
//     };

//     println!("Result: {}", result); // 6

//     // Now watch what happens with a semicolon
//     let result2 = {
//         let y = 3;
//         y * 2; // semicolon — throws away the value!
//     }; // this block now produces () — called "unit" — means nothing

//     // result2 is now () — the empty type
//     println!("Result2: {:?}", result2); // prints ()
// }

//
// fn
// fn print_name(name: &str) {
//     // name receives ownership of the String
//     println!("Hello, {}!", name);
// } // name goes out of scope here — String is dropped (freed from memory)

// fn main() {
//     let my_name = String::from("Cove");
//     // my_name OWNS the String "Cove"

//     print_name(&my_name);
//     // my_name MOVES into the function
//     // my_name no longer owns anything
//     // my_name is now invalid

//     println!("{}", my_name); // ❌ ERROR — my_name owns nothing
// }

// Functions returning ownership
// fn create_greeting(name: &str) -> String {
//     let greeting = format!("Hello, {}! Welcome to Rust!", name);
//     greeting
// }

// fn main() {
//     let message = create_greeting("Cove");
//     println!("{}", message);
// }

// A complete example of burrowning and owenership in Rust
// Takes ownership — original becomes invalid after call
fn take_ownership(s: String) {
    println!("I own this now: {}", s);
} // s is dropped here — memory freed

// Borrows — original completely unaffected
fn borrow(s: &String) {
    println!("Just borrowing: {}", s);
} // reference goes away — original untouched

// Returns ownership — caller receives the value
fn give_ownership() -> String {
    String::from("here, take this")
}

fn main() {
    // Copy type — automatic copy, x still valid
    let x = 42;
    take_number(x);
    println!("x is still {}", x); // ✅

    // String — borrowing with &
    let s1 = String::from("hello");
    borrow(&s1);
    println!("s1 still valid: {}", s1); // ✅

    // String — moving ownership
    let s2 = String::from("world");
    take_ownership(s2);
    // println!("{}", s2); // ❌ s2 is gone

    // Receiving ownership from function
    let s3 = give_ownership();
    println!("Received: {}", s3); // ✅
}

fn take_number(n: i32) {
    println!("Got number: {}", n);
}
