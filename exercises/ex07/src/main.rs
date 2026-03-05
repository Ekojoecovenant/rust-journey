fn main() {
    let age: u32 = 19;

    let life_stage = if age < 13 {
        "Child"
    } else if age < 18 {
        "Teenager"
    } else if age < 65 {
        "Adult"
    } else {
        "Senior"
    };

    let can_vote = if age >= 18 { "Yes" } else { "No" };

    println!("Age: {}", age);
    println!("Life Stage: {}", life_stage);
    println!("Can Vote: {}", can_vote);
}
