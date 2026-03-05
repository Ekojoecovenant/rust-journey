// fn main() {
//     // IF / ELSE IF / ELSE
//     let temperature = 35;

//     if temperature > 30 {
//         println!("It's hot outside! Stay hydrated 💧");
//     } else if temperature > 20 {
//         println!("Nice weather! Go for a walk 🚶🏻");
//     } else if temperature > 10 {
//         println!("A bit chilly. Wear a jacket 🧥");
//     } else {
//         println!("It's cold! Stay inside ❄️")
//     }

//     let score = 85;

//     // if as an EXPRESSION - assigns a value directly
//     let grade = if score >= 90 {
//         "A"
//     } else if score >= 80 {
//         "B"
//     } else if score >= 70 {
//         "C"
//     } else {
//         "F"
//     }; // semicolon here ends the let statement

//     println!("Score: {} | Grade: {}", score, grade);
// }

// Loop Type 1: loop (infinite loop with manual exit)
// fn main() {
//     let mut count = 0;

//     loop {
//         count += 1;
//         println!("Count: {}", count);

//         if count == 5 {
//             break; // EXIT the loop
//         }
//     }

//     println!("Loop finished at count: {}", count);

//     // loop can RETURN a value - very Rust
//     let mut attempts = 0;

//     let result = loop {
//         attempts += 1;

//         if attempts == 3 {
//             break attempts * 10; // break WITH a value
//         }
//     };

//     println!("Result: {}", result);
// }

// Loop Type 2: while (condition-based loop)
// fn main() {
//     let mut number = 1;

//     while number <= 10 {
//         println!("Number: {}", number);
//         number += 1;
//     }

//     println!("Done! Final number: {}", number);
// }

// Loop Type 3: for (iterator loop - the most used in Rust)
// fn main() {
//     // Loop over a range
//     for i in 1..=5 {
//         // 1..=5 means 1 to 5 INCLUSIVE
//         println!("i = {}", i);
//     }

//     // 1..5 means 1 to 4 (exclusive of 5)
//     for i in 1..5 {
//         println!("i = {}", i); // 1,2,3,4
//     }

//     // Loop over an array
//     let fruits = ["apple", "banana", "mango", "pawpaw"];
//     for fruit in fruits {
//         println!("Fruit: {}", fruit);
//     }

//     // Loop with index using .iter().enumerate()
//     for (index, fruit) in fruits.iter().enumerate() {
//         println!("{}: {}", index, fruit);
//     }
// }

// match - Rust's most powerful control flow
fn main() {
    let day = 3;

    let day_name = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day", // _ is the "catch all" / default
    };

    println!("Day {} is {}", day, day_name);

    // Multi-line matcha arms with {}
    let grade = "A";

    match grade {
        "A" => {
            println!("Excellent work!");
            println!("You make me proud 🥹");
        }
        "B" => {
            println!("Great job!");
            println!("Keep pushing for that A!");
        }
        "F" => {
            println!("We need to talk...");
            println!("Extra DSA problems for you 😤");
        }
        _ => println!("Keep working hard!"),
    }

    // match with a binding - capturing the matched value
    let number = 42;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        n => println!("Got some other number: {}", n), // n captures the value
                                                       // n can be another variable name you want it to be
    }
}
