struct Student {
    id: u32,
    name: String,
    gpa: f64,
}

impl Student {
    fn new(id: u32, name: String, gpa: f64) -> Self {
        Student { id, name, gpa }
    }

    fn find_student(students: &[Student], id: u32) -> Option<&Student> {
        for student in students {
            if student.id == id {
                return Some(student);
            }
        }
        None
    }

    fn find_top_student(students: &[Student]) -> Option<&Student> {
        if students.len() == 0 {
            return None;
        }

        let mut top = 0;
        for i in 1..students.len() {
            if students[i].gpa > students[top].gpa {
                top = i;
            }
        }
        Some(&students[top])
    }

    // i got an error saying...
    /* missing lifetime specifier
    this function's return type contains a borrowed value,
    but the signature does not say whether it is borrowed
    from `students` or `name` */
    // i did their suggested fix and it gave something called a lifetime parameter

    fn find_by_name<'a>(students: &'a [Student], name: &'a str) -> Option<&'a Student> {
        for student in students {
            if student.name == name {
                return Some(student);
            }
        }
        None
    }
}

fn main() {
    let students = [
        Student::new(1, String::from("Cove"), 4.0), // im the highest..hehe
        Student::new(2, String::from("Ada"), 3.95),
        Student::new(3, String::from("Turing"), 3.87),
    ];

    println!("\n=== Student Database ===\n");

    // search by id
    println!(
        "Search by ID {:<2}   : {}",
        1,
        if let Some(student) = Student::find_student(&students, 1) {
            format!("Found - {} (GPA: {:.2})", student.name, student.gpa)
        } else {
            format!("Not found")
        }
    );
    println!(
        "Search by ID {:<2}   : {}",
        99,
        if let Some(student) = Student::find_student(&students, 99) {
            format!("Found - {} (GPA: {:.2})", student.name, student.gpa)
        } else {
            format!("Not found")
        }
    );

    // top student
    println!(
        "Top student       : {}",
        if let Some(student) = Student::find_top_student(&students) {
            format!("{} (GPA: {:.2})", student.name, student.gpa)
        } else {
            format!("No Student available")
        }
    );

    // search by name
    println!(
        "Search \"Ada\"      : {}",
        if let Some(student) = Student::find_by_name(&students, "Ada") {
            format!("Found - {} (GPA: {:.2})", student.name, student.gpa)
        } else {
            format!("Student not found - default student used")
        }
    );
    println!(
        "Search \"Nobody\"   : {}",
        if let Some(student) = Student::find_by_name(&students, "Nobody") {
            format!("Found - {} (GPA: {:.2})", student.name, student.gpa)
        } else {
            format!("Student not found - default student used")
        }
    );
}
