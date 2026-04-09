use std::fmt;

#[derive(Debug, Clone)]
struct Student {
    name: String,
    age: u32,
    grade: f64,
}
#[derive(Debug)]
struct ReportCard {
    student: Student,
    subject: String,
    score: f64,
    max_score: f64,
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Student : {} (Age: {}, Grade: {})",
            self.name, self.age, self.grade
        )
    }
}

impl fmt::Display for ReportCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "=== Report ===\n{}\n{}\n{}\n{}",
            self.student,
            format!("Subject : {}", self.subject),
            format!(
                "Score   : {}/{} ({:.2}%)",
                self.score,
                self.max_score,
                (self.score / self.max_score) * 100.0,
            ),
            format!("Grade   : {}", {
                let percent = (self.score / self.max_score) * 100.0;

                match percent {
                    90.0..=100.0 => 'A',
                    80.0..90.0 => 'B',
                    70.0..80.0 => 'C',
                    60.0..70.0 => 'D',
                    _ => 'F',
                }
            }),
        )
    }
}

fn main() {
    let student = Student {
        name: String::from("Cove"),
        age: 19,
        grade: 95.0,
    };

    let report1 = ReportCard {
        student: student.clone(),
        subject: String::from("Rust Programming"),
        score: 99.0,
        max_score: 100.0,
    };

    let report2 = ReportCard {
        student: student.clone(),
        subject: String::from("TypeScript Programming"),
        score: 100.0,
        max_score: 100.0,
    };

    println!("\n{}", report1);
    println!("\n{:?}", report2);
}
