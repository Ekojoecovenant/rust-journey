// fn main() {
//     let x = 5;
//     let x = x + 1;
//     let x = x * 2;

//     println!("x is {x}");
// }

// Tuple
// fn main() {
//     let student = ("Cove", 202, 3.95, true);

//     // Access by index
//     println!("Name: {}", student.0);
//     println!("Age: {}", student.1);
//     println!("GPA: {}", student.2);
//     println!("Passing: {}", student.3);

//     // Destructuring - unpacking a tuple
//     let (name, age, gpa, passing) = student;
//     println!("{name} is {age} years old with GPA {gpa} ({passing})");
// }

// Array - Same type, fixed size
// fn main() {
//     let _scores = [95, 87, 92, 88, 99]; // Rust infers [i32; 5]

//     // Explicit type annotation
//     let scores: [i32; 5] = [95, 87, 92, 88, 99];

//     // Access by index (starts at 0!)
//     println!("First: {}", scores[0]);
//     println!("Last: {}", scores[4]);
//     println!("Length: {}", scores.len());

//     // Fill with same value
//     let zeros = [0; 10]; // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
//     println!("Zeros: {:.?}", zeros); // {:?} = debug print
// }

// Basic Math Operations
fn main() {
    let a = 20;
    let b = 6;

    println!("Add: {} + {} = {}", a, b, a + b);
    println!("Subtract: {} - {} = {}", a, b, a - b);
    println!("Multiply: {} * {} = {}", a, b, a * b);
    println!("Divide: {} / {} = {}", a, b, a / b); // integer division!
    println!("Remainder: {} % {} = {}", a, b, a % b); // modulo

    // Float division is different!
    let x = 20.0;
    let y = 6.0;
    println!("Float div: {} / {} = {}", x, y, x / y); // 3.333...
}
