/* PART 1: What are struct and WHY do they exist? */

// struct Student {
//     name: String,
//     age: u32,
//     gpa: f64,
//     is_enrolled: bool,
// }

// fn print_student(s: &Student) {
//     // borrow the struct
//     println!("--- Student Info ---");
//     println!("Name     : {}", s.name);
//     println!("Age      : {}", s.age);
//     println!("GPA      : {}", s.gpa);
//     println!("Enrolled : {}", s.is_enrolled);
// }

// fn main() {
//     let student1 = Student {
//         name: String::from("Cove"),
//         age: 19,
//         gpa: 3.95,
//         is_enrolled: true,
//     };

//     print_student(&student1); // borrow — student1 still valid
//     println!("Still have: {}", student1.name); // ✅
// }

// --------------------------------------

// fn create_student(name: &str, age: u32) -> Student {
//     Student {
//         name: String::from(name),
//         age, // shorthand when variable name matches field name
//         gpa: 0.0,
//         is_enrolled: true,
//     }
// }

// fn main() {
//     let s = create_student("Cove", 19);
//     println!("{} is {} years old", s.name, s.age);
// }

// ------------------------------------------------

// fn main() {
//     let student1 = Student {
//         name: String::from("Cove"),
//         age: 19,
//         gpa: 3.95,
//         is_enrolled: true,
//     };

//     // Create student2 with same values except name and age
//     let student2 = Student {
//         name: String::from("Ada"),
//         age: 22,
//         ..student1 // copy remaining fields from student1
//     };

//     println!("{} has GPA {}", student2.name, student2.gpa); // 3.95

//     /* ⚠️ Ownership warning with update syntax: If any field being copied
//     from student1 is a non-Copy type (like String) — student1 gets partially
//     moved and becomes invalid for those fields. Since we're providing name
//     ourselves and gpa/is_enrolled/age are all Copy types — student1 stays valid here.
//     But be careful. 🧠 */
// }

// -------------------------------------------------------------

// struct Color(u8, u8, u8); // RGB color
// struct Point(f64, f64, f64); // 3D coordinate

// fn main() {
//     let red = Color(255, 0, 0);
//     let origin = Point(0.0, 0.0, 0.0);

//     println!("Red: ({}, {}, {})", red.0, red.1, red.2);
//     println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);
// }

// ----------------------------------------------------------

// #[derive(Debug)] // this line tells Rust to auto-generate debug printing
// struct Student {
//     name: String,
//     age: u32,
//     gpa: f64,
//     is_enrolled: bool,
// }

// fn main() {
//     let s = Student {
//         name: String::from("Cove"),
//         age: 19,
//         gpa: 3.95,
//         is_enrolled: true,
//     };

//     println!("{:?}", s); // compact debug print
//     println!("{:#?}", s); // pretty debug print
// }

// =======================================================================
// PART 2: Methods - Giving your structs behavior
// =======================================================================

// #[derive(Debug)]
// struct Student {
//     name: String,
//     age: u32,
//     gpa: f64,
//     is_enrolled: bool,
// }

// impl Student {
//     fn is_passing(&self) -> bool {
//         self.gpa >= 2.0
//     }

//     fn grade_letter(&self) -> &str {
//         match self.gpa as u32 {
//             4 => "A",
//             3 => "B",
//             2 => "C",
//             1 => "D",
//             _ => "F",
//         }
//     }

//     fn introduce(&self) {
//         println!(
//             "Hi! I'm {} and I'm {} years old. My GPA is {}",
//             self.name, self.age, self.gpa
//         );
//     }

//     fn update_gpa(&mut self, new_gpa: f64) {
//         if new_gpa >= 0.0 && new_gpa <= 4.0 {
//             self.gpa = new_gpa;
//             println!("GPA updated to {}", self.gpa);
//         } else {
//             println!("Invalid GPA - must be between 0.0 and 4.0");
//         }
//     }

//     fn enroll(&mut self) {
//         self.is_enrolled = true;
//         println!("{} is now enrolled", self.name);
//     }

//     fn withdraw(&mut self) {
//         self.is_enrolled = false;
//         println!("{} has withdrawn", self.name);
//     }

//     // associate functions dont take "self" at all. they dont operate on an instance
//     // they are called with "::" and not "."
//     fn new(name: &str, age: u32) -> Student {
//         Student {
//             name: String::from(name),
//             age,
//             gpa: 0.0,
//             is_enrolled: true,
//         }
//     }
// }

// fn main() {
//     let mut student1 = Student {
//         name: String::from("Cove"),
//         age: 19,
//         gpa: 3.95,
//         is_enrolled: true,
//     };

//     student1.introduce();
//     println!("Passing? {}", student1.is_passing());
//     println!("Grade: {}", student1.grade_letter());

//     student1.update_gpa(4.0);
//     student1.withdraw();
//     student1.enroll();

//     //
//     let s = Student::new("Cove", 19);
//     println!("{:#?}", s);
// }

// --------------------------------------------------------------

// #[derive(Debug)]
// struct Student {
//     name: String,
//     age: u32,
//     gpa: f64,
//     is_enrolled: bool,
// }

// // mehtods can return Self to enable chaining
// impl Student {
//     fn new(name: &str, age: u32) -> Self {
//         Student {
//             name: String::from(name),
//             age,
//             gpa: 0.0,
//             is_enrolled: false,
//         }
//     }

//     fn with_gpa(mut self, gpa: f64) -> Self {
//         self.gpa = gpa;
//         self
//     }

//     fn enrolled(mut self) -> Self {
//         self.is_enrolled = true;
//         self
//     }
// }

// fn main() {
//     let student = Student::new("Cove", 19).with_gpa(3.95).enrolled();

//     println!("{:#?}", student);
// }

// -------------------------------------------------------------------

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Contructor
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    // Immutable methods - just reading
    fn area(&self) -> f64 {
        self.width * self.width
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn can_contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Mutable method - modifying data
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    // Display method
    fn describe(&self) {
        println!("Rectangle {}x{}", self.width, self.height);
        println!("  Area      : {}", self.area());
        println!("  Perimeter : {}", self.perimeter());
        println!("  Is square : {}", self.is_square());
    }
}

fn main() {
    let mut r1 = Rectangle::new(10.0, 5.0);
    let r2 = Rectangle::new(3.0, 3.0);

    r1.describe();
    println!("Can r1 contain r2? {}", r1.can_contain(&r2));

    r1.scale(2.0);
    println!("\nAfter scaling by 2:");
    r1.describe();

    println!("\nr2:");
    r2.describe();
}
