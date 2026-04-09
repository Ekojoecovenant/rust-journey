/* =============== TRAITS ============= */

// intro to traits
/*
trait Mammal {
    fn speak(&self) -> String;
    fn move_around(&self) -> String;
    fn describe(&self) -> String {
        String::from("I am a mammal")
    }
}

struct Dog;
struct Cat;

impl Mammal for Dog {
    fn speak(&self) -> String {
        String::from("Woof!")
    }
    fn move_around(&self) -> String {
        String::from("Runs on 4 legs")
    }
}

impl Mammal for Cat {
    fn speak(&self) -> String {
        String::from("Meow!")
    }
    fn move_around(&self) -> String {
        String::from("Sneaks gracefully")
    }
    fn describe(&self) -> String {
        String::from("I am a sneaky mammal")
    }
}

fn main() {
    let cat = Cat;
    let dog = Dog;

    // Dog
    println!("{}", dog.speak());
    println!("{}", dog.describe());
    println!("{}", dog.move_around());

    // Cat
    println!("{}", cat.speak());
    println!("{}", cat.move_around());
    println!("{}", cat.describe());
}
*/

// -----------------------------------------

// Using Traits in Functions
/*
trait Mammal {
    fn speak(&self) -> String;

    fn describe(&self) -> String {
        String::from("I am a mammal")
    }
}

struct Dog;
struct Cat;

impl Mammal for Dog {
    fn speak(&self) -> String {
        String::from("Woof!")
    }
}

impl Mammal for Cat {
    fn speak(&self) -> String {
        String::from("Meow!")
    }
    fn describe(&self) -> String {
        String::from("I am a sneaky mammal")
    }
}

/*fn make_it_speak(animal: &impl Mammal) {
    println!("{}", animal.speak());
    // &impl Mammal means - I accept a reference to ANYTHING that implements Mammal.
    // I don't care what type it actually is.
}*/

// Way 2 — Generic syntax (more powerful, more explicit)
fn make_it_speak<T: Mammal>(animal: &T) {
    println!("{}", animal.speak());
}

fn main() {
    let cat = Cat;
    let dog = Dog;

    // Dog
    make_it_speak(&dog);
    make_it_speak(&cat);
}
*/

// -------------------------------------------------------------

// DERIVED TRAITS

/*
    - Debug: Allows "{:?}" and {:#?} printing.
    - Clone: Allows ".clone()" - creates a deep copy of the value.
        Without this you cant call ".clone()" on your struct
    - PartialEq: Allows "==" and "!=" comparisons between two values of the same type.
*/
/*
#[derive(Debug, Clone, PartialEq)]
struct Student {
    name: String,
    grade: f64,
}
*/

// Implement debug manually (Custom Debug)
/*use core::fmt;

struct Student {
    name: String,
    grade: f64,
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Student] {} - {:.2}", self.name, self.grade)
    }
}

fn main() {
    let student = Student {
        name: "Cove".to_string(),
        grade: 99.0,
    };
    println!("{}", student);
}
*/

/*
    Multiple trait bounds.

    What if a function needs a type that implements NOT just one trait — but TWO?

    fn print_and_compare<T: Debug + PartialEq>(a: &T, b: &T) {
        println!("{:?}", a);
        println!("Equal: {}", a == b);
    }

    See that + between Debug and PartialEq?
    That means — "T must implement BOTH of these traits." Not one. Both.
    In impl Trait syntax it looks like this:

    fn print_and_compare(a: &(impl Debug + PartialEq), b: &(impl Debug + PartialEq)) {
        println!("{:?}", a);
        println!("Equal: {}", a == b);
    }

    Same idea. Just different syntax. The + works the same way.
*/

fn main() {}
