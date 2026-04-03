use std::collections::HashSet;

fn main() {
    let rust_class: HashSet<&str> = ["Cove", "Ada", "Turing", "Grace", "Linus"]
        .iter()
        .cloned()
        .collect();
    let python_class: HashSet<&str> = ["Ada", "Grace", "Bjarne", "Guido"]
        .iter()
        .cloned()
        .collect();
    let webdev_class: HashSet<&str> = ["Cove", "Bjarne", "Grace", "Dennis"]
        .iter()
        .cloned()
        .collect();

    println!("=== Course Enrollment Analysis ===");
    println!("{:<12} : {:?}", "Rust class", rust_class);
    println!("{:<12} : {:?}", "Python class", python_class);
    println!("{:<12} : {:?}", "WebDev class", webdev_class);

    let first_union: HashSet<&str> = python_class.union(&webdev_class).cloned().collect();
    let unique_students_total: HashSet<&str> = rust_class.union(&first_union).cloned().collect();

    println!("\nUnique students total  : {}", unique_students_total.len());

    let rust_and_python: HashSet<_> = rust_class.intersection(&python_class).cloned().collect();
    println!("In Rust AND Python     : {:?}", &rust_and_python);

    let rust_not_python: HashSet<_> = rust_class.difference(&python_class).cloned().collect();
    println!("In Rust NOT Python     : {:?}", rust_not_python);

    let all_three: HashSet<_> = webdev_class
        .intersection(&rust_and_python)
        .cloned()
        .collect();
    println!("In ALL three classes   : {:?}", all_three);

    let largest_class = {
        let rust_len = rust_class.len();
        let webdev_len = webdev_class.len();
        let python_len = python_class.len();

        if rust_len > webdev_len && rust_len > python_len {
            format!(
                "Rust ({} {})",
                rust_len,
                if rust_len != 1 { "students" } else { "student" }
            )
        } else {
            if webdev_len > python_len {
                format!(
                    "WebDev ({} {})",
                    webdev_len,
                    if webdev_len > 1 {
                        "students"
                    } else {
                        "student"
                    }
                )
            } else {
                format!(
                    "Python ({} {})",
                    python_len,
                    if python_len > 1 {
                        "students"
                    } else {
                        "student"
                    }
                )
            }
        }
    };

    println!("Largest class          : {}", largest_class);
}
