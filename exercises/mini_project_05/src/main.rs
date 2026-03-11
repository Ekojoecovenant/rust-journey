fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_chars(text: &str) -> usize {
    text.chars().count()
}

fn count_vowels(text: &str) -> usize {
    let mut count: usize = 0;
    for i in text.chars() {
        if i == 'a'
            || i == 'e'
            || i == 'i'
            || i == 'o'
            || i == 'u'
            || i == 'A'
            || i == 'E'
            || i == 'I'
            || i == 'O'
            || i == 'U'
        {
            count += 1;
        }
    }
    count
}

fn longest_word(text: &str) -> &str {
    let mut largest: &str = "";
    for i in text.split_whitespace() {
        if i.len() > largest.len() {
            largest = i;
        }
    }
    largest
}

fn is_palindrome_sentence(text: &str) -> bool {
    // let text = text.to_lowercase();
    // for (front, front_val) in text.chars().enumerate() {
    //     let back = text.len() - front;
    //     if back > front {
    //         let back_val = text.chars().nth(back);
    //         if back_val != Some(front_val) {
    //             return false;
    //         }
    //     }
    // }
    // true
    let text = text.to_lowercase();
    let mut characters = text.chars();

    while let (Some(front), Some(back)) = (characters.next(), characters.next_back()) {
        if front != back {
            return false;
        }
    }
    true
}

fn most_common_char(text: &str) -> char {
    let mut counts = [0u32; 128];
    for c in text.chars() {
        if (c as usize) < 128 {
            counts[c as usize] += 1;
        }
    }

    let mut max_count = 0u32;
    let mut max_char = ' ';

    for i in 32..128usize {
        if counts[i] > max_count {
            max_count = counts[i];
            max_char = i as u8 as char;
        }
    }
    max_char
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog";

    println!("=== Text Analyzer ===");
    println!("Text     : {}\n", text);

    println!("Words    : {}", count_words(text));
    println!("Chars    : {}", count_chars(text));
    println!("Vowels   : {}", count_vowels(text));
    println!("Longest  : {}", longest_word(text));
    println!("Palindrome Sentence : {}", is_palindrome_sentence(text));
    println!("Most Common Char : {}", most_common_char(text));
}
