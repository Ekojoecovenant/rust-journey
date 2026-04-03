fn highest(grades: &[i32]) -> i32 {
    let mut highest = 0;
    for i in 1..grades.len() {
        if grades[i] > grades[highest] {
            highest = i;
        }
    }
    grades[highest]
}

fn lowest(grades: &[i32]) -> i32 {
    let mut lowest = 0;
    for i in 1..grades.len() {
        if grades[i] < grades[lowest] {
            lowest = i;
        }
    }
    grades[lowest]
}

fn average(grades: &[i32]) -> f64 {
    let mut sum = 0;
    for grade in grades {
        sum += *grade;
    }
    sum as f64 / grades.len() as f64
}

fn passing(grades: &[i32]) -> Vec<i32> {
    let mut passed = Vec::new();
    for grade in grades {
        if *grade >= 70 {
            passed.push(*grade);
        }
    }
    passed
}

fn failing(grades: &[i32]) -> Vec<i32> {
    let mut failed = Vec::new();
    for grade in grades {
        if *grade < 70 {
            failed.push(*grade);
        }
    }
    failed
}

fn main() {
    let mut grades: Vec<i32> = Vec::new();

    for value in [85, 92, 78, 95, 88, 73, 96, 81] {
        grades.push(value);
    }
    println!("=== Gradebook ===");
    println!("Grades    : {:?}", grades);
    println!("Popped    : {}", {
        match grades.pop() {
            None => format!("Empty"),
            Some(value) => format!("{}", value),
        }
    });
    println!("Remaining : {:?}", grades);

    println!("\nAnalysis:");
    println!("Highest  : {}", highest(&grades));
    println!("Lowest   : {}", lowest(&grades));
    println!("Average  : {:.2}", average(&grades));
    println!("Passing  : {:?}", passing(&grades));
    println!("Failing  : {:?}", failing(&grades));
}
