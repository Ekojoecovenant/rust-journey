fn main() {
    let score: u32 = 87;

    let grade = match score {
        90..=100 => "A",
        80..90 => "B",
        70..80 => "C",
        _ => "F",
    };

    let message = match grade {
        "A" => "You make me proud 🥹",
        "B" => "Keep pushing for that A!",
        "C" => "Not your best but you passed",
        "F" => "Keep working hard",
        _ => "We need to talk!",
    };

    let category = match grade {
        "A" => "Perfect",
        "B" => "Excellent",
        "C" => "Passing",
        "F" => "Failing",
        _ => "Error",
    };

    println!("--- Report Card ---");
    println!("Score:    {}", score);
    println!("Grade:    {}", grade);
    println!("Message:  {}", message);
    println!("Category: {}", category);
}
