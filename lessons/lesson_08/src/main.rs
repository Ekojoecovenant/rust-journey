/*=========== PART 1: What are Enums and WHY are they special in Rust */

// basic enum definition
/*
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let heading = Direction::North;

    match heading {
        Direction::North => println!("Going North!"),
        Direction::South => println!("Going South!"),
        Direction::East => println!("Going East!"),
        Direction::West => println!("Going West!"),
    }
}
*/

// ---------------------------------------------

// Enums with Data and extracting data with match
/*
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
    Point,
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14159 * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Triangle(a, b, c) => {
            // Heron's formula
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
        Shape::Point => 0.0,
    }
}

fn main() {
    let shapes = [
        Shape::Circle(5.0),
        Shape::Rectangle(10.0, 4.0),
        Shape::Triangle(3.0, 4.0, 5.0),
        Shape::Point,
    ];

    for shape in &shapes {
        println!("Area: {:.2}", area(shape));
    }
}
*/

// ------------------------------------------

// Enums with Struct-like structs
/*
enum Message {
    Quit,                    // no data
    Move { x: i32, y: i32 }, // named fields
    Write(String),           // single value
    ChangeColor(u8, u8, u8), // multiple values
}

fn process(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quitting!");
        }
        Message::Move { x, y } => {
            println!("Moving to ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("Writing: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Color: rgb({}, {}, {})", r, g, b);
        }
    }
}

fn main() {
    process(Message::Move { x: 10, y: 20 });
    process(Message::Write(String::from("hello")));
    process(Message::ChangeColor(255, 128, 0));
    process(Message::Quit);
}
*/

// -------------------------------------

// Methods on Enums

/*enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }

    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }

    fn is_safe_to_go(&self) -> bool {
        match self {
            TrafficLight::Green => true,
            _ => false,
        }
    }
}

fn main() {
    let light = TrafficLight::Red;

    println!("Current  : Red");
    println!("Duration : {} seconds", light.duration());
    println!("Safe?    : {}", light.is_safe_to_go());

    let next = light.next();
    println!("\nNext     : Green");
    println!("Duration : {} seconds", next.duration());

    let next = next.next();
    println!("\nNext     : Yello");
    println!("Duration : {} seconds", next.duration());
}
*/

/* ========== PART 2: "Option<T>" - Rust's answer to "null" ============= */

// fn main() {
//     let some_number: Option<i32> = Some(42);
//     let some_text: Option<String> = Some(String::from("hello"));
//     let no_number: Option<i32> = None;
//     let no_text: Option<String> = None;

//     println!("{:?}", some_number); // Some(42)
//     println!("{:?}", no_number); // None
// }

// ------------------------------------------------------------

// Using Option with match
/*fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn main() {
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);

    match result1 {
        Some(value) => println!("10 / 2 = {}", value),
        None => println!("Cannot divide by zero"),
    }

    match result2 {
        Some(value) => println!("10 / 0 = {}", value),
        None => println!("Cannot divide by zero"),
    }
}
*/

// ------------------------------------------------------
/*
fn find_first_even(arr: &[i32]) -> Option<i32> {
    for &val in arr {
        if val % 2 == 0 {
            return Some(val);
        }
    }
    None
}

fn main() {
    let numbers1 = [1, 3, 5, 4, 7];
    let numbers2 = [1, 3, 5, 7, 9];

    match find_first_even(&numbers1) {
        Some(n) => println!("First even: {}", n),
        None => println!("No even numbers found"),
    }

    match find_first_even(&numbers2) {
        Some(n) => println!("First even: {}", n),
        None => println!("No even numbers found"),
    }

    if let Some(n) = find_first_even(&numbers1) {
        println!("Found: {}", n);
    } else {
        println!("Not found");
    }

    // -----------------
    println!("");
    let some_val: Option<i32> = Some(42);
    let no_val: Option<i32> = None;

    // is_some() and is_none() — check without extracting
    println!("{}", some_val.is_some()); // true
    println!("{}", no_val.is_none()); // true

    // unwrap_or() — get value OR a default
    println!("{}", some_val.unwrap_or(0)); // 42
    println!("{}", no_val.unwrap_or(0)); // 0  ← default, no panic!

    // unwrap_or_else() — get value OR compute a default
    println!("{}", no_val.unwrap_or_else(|| 99)); // 99
}
*/

// -----------------------------------------------

/*
fn make_default() -> i32 {
    println!("computing default...");
    99
}

fn main() {
    let has_value: Option<i32> = Some(42);
    let no_value: Option<i32> = None;

    // unwrap_or — ALWAYS computes default
    println!("--- unwrap_or ---");
    let a = has_value.unwrap_or(make_default());
    // "computing default..." prints EVEN THOUGH has_value is Some!
    println!("a = {}", a); // 42

    let b = no_value.unwrap_or(make_default());
    // "computing default..." prints
    println!("b = {}", b); // 99

    // unwrap_or_else — LAZY, only computes when needed
    println!("\n--- unwrap_or_else ---");
    let c = has_value.unwrap_or_else(|| make_default());
    // "computing default..." does NOT print — skipped entirely!
    println!("c = {}", c); // 42

    let d = no_value.unwrap_or_else(|| make_default());
    // "computing default..." prints — None so it runs
    println!("d = {}", d); // 99
}
*/

/*============== PART 3: Result<T, E> - Rust's error handling ============= */

/*
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);

    match result1 {
        Ok(value) => println!("10 / 2 = {}", value),
        Err(error) => println!("Error: {}", error),
    }

    match result2 {
        Ok(value) => println!("10 / 0 = {}", value),
        Err(error) => println!("Error: {}", error),
    }

    println!("");
    let result = divide(10.0, 2.0);

    if let Ok(value) = result {
        println!("Success: {}", value);
    }

    // With else
    if let Ok(value) = divide(10.0, 0.0) {
        println!("Success: {}", value);
    } else {
        println!("Something went wrong");
    }
}
*/

// ------------------------------------------

// without ? - verbose and repetive
/*fn process(s: &str) -> Result<f64, String> {
    let parsed = parse_number(s);
    let number = match parsed {
        Ok(n)  => n,
        Err(e) => return Err(e), // propagate error up
    };

    let result = divide(number, 2.0);
    let divided = match result {
        Ok(n)  => n,
        Err(e) => return Err(e), // propagate error up
    };

    Ok(divided)
}*/

// With ? — clean and elegant
/*fn process(s: &str) -> Result<f64, String> {
    let number = parse_number(s)?;  // if Err — return immediately
    let divided = divide(number, 2.0)?;  // if Err — return immediately
    Ok(divided)
}*/

// -------------------------------------

// Custom Error Types

/*
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn sqare_root(n: f64) -> Result<f64, MathError> {
    if n < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(n.sqrt())
    }
}

fn main() {
    let operations = [
        divide(10.0, 2.0),
        divide(10.0, 0.0),
        sqare_root(16.0),
        sqare_root(-4.0),
    ];

    for op in &operations {
        match op {
            Ok(value) => println!("✅ Result: {:.2}", value),
            Err(MathError::DivisionByZero) => println!("❌ Cannot divide by zero"),
            Err(MathError::NegativeSquareRoot) => println!("❌ Cannot sqrt negative number"),
            Err(MathError::Overflow) => println!("❌ Overflow occurred"),
        }
    }
}
*/

// ---------------------------------------

// Result methods
/*fn main() {
    let ok: Result<i32, String> = Ok(42);
    let err: Result<i32, String> = Err(String::from("oops"));

    // is_ok() and is_err()
    println!("{}", ok.is_ok());
    println!("{}", err.is_err());

    // unwrap_or - safe default on error
    println!("{}", ok.unwrap_or(0));
    println!("{}", err.unwrap_or(0));

    let err: Result<i32, String> = Err(String::from("oops"));
    // unwrap_or_else - laxy default
    println!(
        "{}",
        err.unwrap_or_else(|e| {
            println!("Got error: {}", e);
            -1
        })
    );
}*/

// --------------------------

// Option and Result
fn find_and_parse(arr: &[&str], index: usize) -> Result<i32, String> {
    // arr.get(index) returns Option<&&str>
    // if index out of bounds → None
    let item = arr.get(index).ok_or(String::from("Index out of bounds"))?;
    //  ^^^^^^^^
    //  converts Option to Result
    //  None becomes Err, Some becomes Ok
    //  ? propagates the error if Err

    item.parse::<i32>()
        .map_err(|e| format!("Parse error: {}", e))
    //  converts the parse error type to our String error type
}

fn main() {
    let data = ["42", "hello", "100"];

    match find_and_parse(&data, 0) {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match find_and_parse(&data, 1) {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match find_and_parse(&data, 5) {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
