fn char_frequency(text: &str) -> [u32; 26] {
    let mut freq = [0; 26];
    for c in text.to_lowercase().chars() {
        if c >= 'a' && c <= 'z' {
            let idx = (c as u8 - b'a') as usize;
            // println!("{} : {}", c, idx);
            freq[idx] += 1;
        }
    }
    freq
}

fn print_frequency(freq: &[u32; 26]) {
    for (i, v) in freq.iter().enumerate() {
        if *v >= 1 {
            let c = (i as u8 + b'a') as char;
            println!("{}: {}", c, *v);
        }
    }
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog";
    let freq = char_frequency(text);

    println!("Character Frequency:");
    print_frequency(&freq);
}
