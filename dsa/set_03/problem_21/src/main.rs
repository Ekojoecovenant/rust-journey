// i also stole this one from problem 18...after all...i solved it myself so i basically reused my solution..hehe
fn build_frequency(s: &str) -> [u32; 26] {
    let mut freq = [0; 26];
    for c in s.to_lowercase().chars() {
        if c >= 'a' && c <= 'z' {
            let idx = (c as u8 - b'a') as usize;
            freq[idx] += 1;
        }
    }
    freq
}

fn is_anagram(s1: &str, s2: &str) -> bool {
    let s1 = build_frequency(s1);
    let s2 = build_frequency(s2);

    for i in 0..26 {
        if s1[i] != s2[i] {
            return false;
        }
    }
    true
}

fn is_match(b: bool) -> String {
    if b {
        String::from("true  ✅")
    } else {
        String::from("false ❌")
    }
}

fn main() {
    println!(
        "{:<8} & {:<8} : {}",
        "listen",
        "silent",
        is_match(is_anagram("listen", "silent"))
    );
    println!(
        "{:<8} & {:<8} : {}",
        "hello",
        "world",
        is_match(is_anagram("hello", "world"))
    );
    println!(
        "{:<8} & {:<8} : {}",
        "Rust",
        "tsuR",
        is_match(is_anagram("Rust", "tsuR"))
    );
    println!(
        "{:<8} & {:<8} : {}",
        "anagram",
        "nagaram",
        is_match(is_anagram("anagram", "nagaram"))
    );
    println!(
        "{:<8} & {:<8} : {}",
        "rat",
        "car",
        is_match(is_anagram("rat", "car"))
    );
    // i also checked for cases with different character case
    println!(
        "{:<8} & {:<8} : {}",
        "Rat",
        "Tar",
        is_match(is_anagram("Rat", "Tar")) // it returned true
    );
}
