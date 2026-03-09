fn encrypt(text: &str, shift: u8) -> String {
    let mut result = String::new();

    for c in text.chars() {
        let shifted = match c {
            'A'..='Z' => (((c as u8 - b'A') + shift) % 26 + b'A') as char,
            'a'..='z' => (((c as u8 - b'a') + shift) % 26 + b'a') as char,
            _ => c,
        };

        result.push(shifted);
    }

    result
}

fn decrypt(text: &str, shift: u8) -> String {
    let mut result = String::new();

    for c in text.chars() {
        let shifted = match c {
            'A'..='Z' => (((c as u8 - b'A') + 26 - shift) % 26 + b'A') as char,
            'a'..='z' => (((c as u8 - b'a') + 26 - shift) % 26 + b'a') as char,
            _ => c,
        };

        result.push(shifted);
    }

    result
}

fn main() {
    let text = "Hello, Rust World! 123";
    let shift = 3;
    let encrypted = encrypt(text, shift);
    let decrypted = decrypt(&encrypted, shift);

    println!("Original  : {}", text);
    println!("Shift     : {}", shift);
    println!("Encrypted : {}", encrypted);
    println!("Decrypted : {}", decrypted);
}
