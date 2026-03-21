struct Course {
    name: String,
    code: String,
    credits: u32,
    grade: f64,
}

struct Student {
    name: String,
    id: u32,
    age: u32,
    courses: [Course; 3], // fixed array of 3 courses
}

impl Course {
    fn new(name: &str, code: &str, credits: u32, grade: f64) -> Self {
        Course {
            name: String::from(name),
            code: String::from(code),
            credits,
            grade,
        }
    }

    fn grade_letter(&self) -> &str {
        match self.grade {
            90.0..=100.0 => "A",
            80.0..90.0 => "B",
            70.0..80.0 => "C",
            60.0..70.0 => "D",
            50.0..60.0 => "E",
            _ => "F",
        }
    }

    fn is_passing(&self) -> bool {
        self.grade >= 50.0
    }
}

impl Student {
    fn new(name: &str, id: u32, age: u32, courses: [Course; 3]) -> Self {
        Student {
            name: String::from(name),
            id,
            age,
            courses,
        }
    }

    fn average_grade(&self) -> f64 {
        let mut total = 0.0;
        for course in &self.courses {
            total += course.grade;
        }
        (total / 3.0 * 100.0).round() / 100.0
    }

    fn total_credits(&self) -> u32 {
        let mut total = 0u32;
        for course in &self.courses {
            total += course.credits;
        }
        total
    }

    fn is_on_honor_roll(&self) -> bool {
        self.average_grade() >= 80.0
    }

    fn print_transcript(&self) {
        println!("=== Student Transcript ===");
        println!("Name       : {}", self.name);
        println!("ID         : {}", self.id);
        println!("Age        : {}", self.age);

        println!("\nCourses:");
        for course in &self.courses {
            println!(
                "  {:<19} ({}) - Grade: {} - {} {}",
                course.name,
                course.code,
                course.grade,
                course.grade_letter(),
                if course.is_passing() { "✅" } else { "❌" }
            );
        }

        println!("\nAverage    : {}", self.average_grade());
        println!("Credits    : {}", self.total_credits());
        let is_honor = self.is_on_honor_roll();
        println!(
            "Honor Roll : {} {}",
            is_honor,
            if is_honor { "🏆" } else { "⛔" }
        );
    }
}

fn main() {
    let course1 = Course::new("Rust Programming", "CSC101", 3, 95.0);
    let course2 = Course::new("Data Structures", "CSC102", 3, 88.0);
    let course3 = Course::new("Web Development", "CSC103", 3, 92.0);

    let student1 = Student::new("Cove", 20240001, 19, [course1, course2, course3]);

    student1.print_transcript();
}
