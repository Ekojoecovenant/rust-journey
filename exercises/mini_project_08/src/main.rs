use std::collections::{HashMap, HashSet};

struct Student {
    name: String,
    id: u32,
}

struct GradeBook {
    students: Vec<Student>,
    grades: HashMap<u32, Vec<f64>>,
    honor_roll: HashSet<u32>,
}

impl GradeBook {
    fn new() -> Self {
        GradeBook {
            students: Vec::new(),
            grades: HashMap::new(),
            honor_roll: HashSet::new(),
        }
    }

    fn add_student(&mut self, name: &str, id: u32) {
        let new_student = Student {
            name: name.to_string(),
            id,
        };
        self.students.push(new_student);
    }

    fn add_grade(&mut self, student_id: u32, grade: f64) {
        let grades = self.grades.entry(student_id).or_insert(vec![]);
        grades.push(grade);
    }

    fn get_average(&self, student_id: u32) -> Option<f64> {
        let student_grades = self.grades.get(&student_id)?;

        let mut sum = 0.0;
        for grade in student_grades {
            sum += *grade;
        }
        Some(sum / student_grades.len() as f64)
    }

    fn update_honor_roll(&mut self) {
        // let student_verified = ;
        for student_id in self.grades.keys() {
            let average = self.get_average(*student_id);

            match average {
                None => {}
                Some(avg) => {
                    if avg >= 85.0 {
                        self.honor_roll.insert(*student_id);
                    }
                }
            }
        }
    }

    fn print_report(&self) {
        println!("\n=== Class Report ===");
        println!(
            "{:<10} {:<7} {:<27} {:<9} {}",
            "Name", "ID", "Grades", "Average", "Honor Roll"
        );

        for student in &self.students {
            let Some(grade) = self.grades.get(&student.id) else {
                return;
            };

            let Some(average) = self.get_average(student.id) else {
                return;
            };

            let is_honor = self.honor_roll.contains(&student.id);

            println!(
                "{:<10} {:<7} {:<27} {:<9.2} {}",
                student.name,
                student.id,
                format!("{:?}", grade),
                average,
                if is_honor { "⭐" } else { "" }
            )
        }
    }
}

fn main() {
    println!("=== Grade Management System ===");

    let mut gradebook1 = GradeBook::new();

    // adding students
    print!("Added students: ");
    gradebook1.add_student("Cove", 1001);
    print!("Cove, ");
    gradebook1.add_student("Ada", 1002);
    print!("Ada, ");
    gradebook1.add_student("Turing", 1003);
    print!("Turing, ");
    gradebook1.add_student("Grace", 1004);
    print!("Grace, ");
    gradebook1.add_student("Linus", 1005);
    println!("Linus");

    // adding grades
    println!("\nAdding grades...");
    for grade in vec![95.0, 98.0, 92.0, 97.0] {
        // i got the highest score this time...hahahahaha
        gradebook1.add_grade(1001, grade);
    }

    for grade in vec![78.0, 82.0, 79.0, 85.0] {
        gradebook1.add_grade(1002, grade);
    }

    for grade in vec![92.0, 88.0, 95.0, 91.0] {
        gradebook1.add_grade(1003, grade);
    }

    for grade in vec![65.0, 70.0, 68.0, 72.0] {
        gradebook1.add_grade(1004, grade);
    }

    for grade in vec![88.0, 85.0, 90.0, 87.0] {
        gradebook1.add_grade(1005, grade);
    }

    // updating honor roll
    gradebook1.update_honor_roll();

    // report
    gradebook1.print_report();

    // class average
    let mut total_grades = 0.0;
    let total_students = gradebook1.students.len() as f64;
    for student in &gradebook1.students {
        total_grades += gradebook1.get_average(student.id).unwrap_or(0.0);
    }

    let class_average = total_grades / total_students;
    println!("\nClass Average : {}", class_average);

    // top student
    let mut top_grade = 0.0;
    let mut top_student = "".to_string();

    for student in &gradebook1.students {
        let grade = gradebook1.get_average(student.id).unwrap_or(0.0);
        if top_grade < grade {
            top_grade = grade;
            top_student = student.name.clone();
        }
    }

    println!("Top Student   : {} ({:.2})", top_student, top_grade);

    // honor roll
    println!(
        "Honor Roll    : {} {}",
        gradebook1.honor_roll.len(),
        if gradebook1.honor_roll.len() != 1 {
            "students"
        } else {
            "student"
        }
    );
}
