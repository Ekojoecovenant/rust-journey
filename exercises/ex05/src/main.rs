fn main() {
    let temperature: f64 = 36.6;
    let heart_rate: u32 = 72;
    let is_healthy: bool = true;
    let blood_type: char = 'O';

    let temperature_fahrenheit = (temperature * 9.0 / 5.0) + 32.0;

    println!("=== Health Report ===");
    println!(
        "Temperature : {}°C / {}°F",
        temperature, temperature_fahrenheit
    );
    println!("Heart Rate  : {} bpm", heart_rate);
    println!("Blood Type  : {}", blood_type);
    println!(
        "Status      : {}",
        if is_healthy { "Healthy" } else { "Unhealthy" }
    );
    // I noticed that we didnt use the is_healthy variable yet...so i got a warning in the console...it ran tho
}
