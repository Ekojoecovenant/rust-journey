const MAX_AGE: u32 = 120;
const APP_VERSION: &str = "1.0.0";

fn main() {
    let name: &str = "Cove";
    let age: i32 = 19;
    let height: f64 = 1.72;
    let is_student: bool = true;
    let weight: f64 = 70.0;

    let bmi = weight / (height * height);
    let bmi = (bmi * 100.0).round() / 100.0; // shadow to round to 2 decimal places

    println!("╔══=═════════════════════=═══╗");
    println!("║     PERSONAL INFO CARD     ║");
    println!("╠═=════════════════════════=═╣");
    println!("║  MAX AGE     : {:<12}║", MAX_AGE);
    println!("║  APP VERSION : {:<12}║", APP_VERSION);
    println!("║  Name        : {:<12}║", name);
    println!("║  Age         : {:<12}║", age);
    println!("║  Height      : {:<12}║", height);
    println!(
        "║  Is Student  : {:<12}║",
        if is_student { "Yes" } else { "No" }
    );
    println!("║  Weight      : {:<12}║", weight);
    println!("║  BMI         : {:<12}║", bmi);
    println!("╚═════════════════==═════════╝");
}
