/* =========== PART 1: Vec<T> - The Dynamic Array =========== */

// Creating a Vec
/*
fn main() {
    // Method 1 - empty Vec with type annotation
    let mut numbers: Vec<i32> = Vec::new();

    // Method 2 - empty Vec with types inferred later
    let mut names = Vec::new();
    names.push(String::from("Cove")); // rust infers Vec<String>

    // Method 3 - vec! macro with initial values
    let scores = vec![95, 87, 92, 88, 99];
    //                     ^^^^ macro shorthand - most common in practice

    println!("{:?}", numbers);
    println!("{:?}", names);
    println!("{:?}", scores);
}
*/

// ---------------------------------

// Adding and removing elements
/*
fn main() {
    let mut v: Vec<i32> = Vec::new();

    // push - add to the END
    v.push(10);
    v.push(20);
    v.push(30);
    println!("{:?}", v);

    // pop - remove from END , returns Option<T>
    let last = v.pop();
    println!("{:?}", last);
    println!("{:?}", v);

    // pop on empty Vec returns None - safe!
    let mut empty: Vec<i32> = Vec::new();
    println!("{:?}", empty.pop());
}
*/

// ------------------------------------

// Accessing elements
/*
fn main() {
    let v = vec![10, 20, 30, 40, 50];

    // Method 1 - indexing (can panic!)
    let third = v[2];
    println!("Third: {}", third);

    // Method 2 - .get() returns Option (safe!)
    let third = v.get(2);
    println!("{:?}", third);

    let out_of_bounds = v.get(99);
    println!("{:?}", out_of_bounds);

    // let _crash = v[99];
}
*/

// ---------------------------------------

// Iterating over a Vec
/*
fn main() {
    let v = vec![10, 20, 30, 40, 50];

    // Immutable iteration - borrowing each element
    for val in &v {
        println!("{}", val);
    }
    // v still valid after this loop

    // Mutable iteration - can modify each element
    let mut v2 = vec![1, 2, 3, 4, 5];
    for val in &mut v2 {
        *val *= 2; // dereference to modify
    }
    println!("{:?}", v2);

    // Consuming iteration - moves ownership
    let v3 = vec![1, 2, 3];
    for val in v3 {
        println!("{}", val);
    }
    // v3 is gone after this - ownership is consumed
}
*/

// -----------------------------------

// Useful Vec methods
/*
fn main() {
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];

    // length
    println!("Length: {}", v.len());

    // is it empty?
    println!("Empty: {}", v.is_empty());

    // contains a value?
    println!("Has 9: {}", v.contains(&9));

    // sort
    v.sort();
    println!("Sorted: {:?}", v);

    // dedup - remove consecutive duplicates (sort first!)
    v.dedup();
    println!("Deduped: {:?}", v);

    // reverse
    v.reverse();
    println!("Reversed: {:?}", v);

    // retain - keep only elements matching condition
    v.retain(|&x| x > 3);
    println!("Retained >3: {:?}", v);

    // extend - add multiple elements
    let mut a = vec![1, 2, 3];
    a.extend([4, 5, 6]);
    println!("Extended: {:?}", a);
}
*/

// ----------------------------------------

// Vec and Ownership - Important
/*
fn main() {
    let v = vec![String::from("hello"), String::from("world")];

    // This MOVES ownership - v is consumed
    for s in v {
        println!("{}", s);
    }
    // println!("{:?}", v); // v is gone

    // This BORROWS - v stays valid
    let v2 = vec![String::from("hello"), String::from("world")];

    for s in &v2 {
        println!("{}", s);
    }
    println!("{:?}", v2); // v2 still here
}
*/

// ------------------------------------------

// Vec in functions
/*
// Takes ownership - Vec is gone after call
fn print_and_consume(v: Vec<i32>) {
    for val in v {
        println!("{}", val);
    }
}

// Borrows - Vec still valid after call
fn sum(v: &Vec<i32>) -> i32 {
    let mut total = 0;
    for val in v {
        total += val;
    }
    total
}

// Even better - take &[i32] slice instead of &Vec<i32>
// Works for both Vec AND arrays!
fn sum_slice(v: &[i32]) -> i32 {
    let mut total = 0;
    for &val in v {
        total += val;
    }
    total
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("Sum: {}", sum(&v)); // v still valid
    println!("Sum: {}", sum_slice(&v)); // v still valid

    let arr = [1, 2, 3, 4, 5];
    println!("Sum: {}", sum_slice(&arr)); // works for arrays too
}
*/

/* ================ PART 2: HashMap<K, V> -- Key-Value Storage ================= */

// Creating a HashMap
/*use std::collections::HashMap;

fn main() {
    // Method 1 - empty HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Method 2 - type inferred from first insert
    let mut scores = HashMap::new();
    scores.insert(String::from("Cove"), 95); // Rust infers HashMap<String, i32>
    scores.insert(String::from("Ada"), 92);
    scores.insert(String::from("Turing"), 87);

    println!("{:?}", scores);
}
*/

// ------------------------------------------

// Inserting andn updating
/*
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    // insert - adds new OR overwrites existing
    map.insert("Cove", 95);
    map.insert("Cove", 100); // overwrites 95!
    println!("{:?}", map.get("Cove"));

    // entry API - insert ONLY if key doesn't exist
    map.entry("Ada").or_insert(92);
    map.entry("Ada").or_insert(50); // Ada already exists - ignored
    println!("{:?}", map.get("Ada"));

    // entry API - modify existing value
    let cove_score = map.entry("Cove").or_insert(0);
    *cove_score += 5; // dereference to modify
    println!("{:?}", map.get("Cove"));
}
*/

// -------------------------------------------------

// Reading values
/*use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Cove"), 95);
    scores.insert(String::from("Ada"), 92);

    // get- returns Option<&V>
    match scores.get("Cove") {
        Some(score) => println!("Cove's score: {}", score),
        None => println!("Not found"),
    }

    // contains_key - check existence (bool)
    println!("{}", scores.contains_key("Ada"));
    println!("{}", scores.contains_key("Nobody"));

    // get with unwrap_or - with default
    let score = scores.get("Nobody").unwrap_or(&0);
    println!("Nobody's score: {}", score);

    // direct index - panics if missing
    // let s = scores["Nobody"];
    let s = scores["Cove"];
    println!("{}", s);
}
*/

// -----------------------------------------------

// Removing entries
/*
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    // remove - returns Option<V>
    let removed = map.remove("b");
    println!("{:?}", removed);
    println!("{:?}", map);

    let not_there = map.remove("z");
    println!("{:?}", not_there);
}
*/

// --------------------------------------------------

// Iterating over HashMap
/*
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Cove", 95);
    scores.insert("Ada", 92);
    scores.insert("Turing", 87);

    // Iterate over key-value pairs
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // Iterate over keys only
    for name in scores.keys() {
        println!("Student: {}", name);
    }

    // Iterate over values only
    for score in scores.values() {
        println!("Score: {}", score);
    }

    // Mutable values
    for score in scores.values_mut() {
        *score += 5;
    }
    println!("{:?}", scores);
}
*/

// -------------------------------------------

// HashMap and Ownership
/*
use std::collections::HashMap;

fn main() {
    let key = String::from("name");
    let value = String::from("Cove");

    let mut map = HashMap::new();
    map.insert(key, value);
    // key and value are MOVED into the map

    // Copy types are copies not moved
    let mut map2 = HashMap::new();
    let x = 5;
    map2.insert(x, 10);
    println!("{}", x); // still valid
}
*/

// ------------------------------------------

// Word Counter - HashMap example
/*
use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<&str, u32> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    counts
}

fn main() {
    let text = "hello world hello rust world hello";
    let counts = word_count(text);

    for (word, count) in &counts {
        println!("{}: {}", word, count);
    }
}
*/

/* ============ PART 3: HashSet<T> - Unique values only =============== */

// Creating a HashSet
/*
use std::collections::HashSet;

fn main() {
    // Empty HashSet
    let mut set: HashSet<i32> = HashSet::new();

    // From an array - duplicates removed automatically
    let set: HashSet<i32> = [1, 2, 3, 2, 1, 4].iter().cloned().collect();
    println!("{:?}", set);
}
*/

// ----------------------------------

// Core operations
/*
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();

    // insert - returns bool! true if NEW, false if already existed
    println!("{}", set.insert(1));
    println!("{}", set.insert(2));
    println!("{}", set.insert(1));

    // contains - check membership
    println!("{}", set.contains(&1));
    println!("{}", set.contains(&99));

    // remove - returns bool, true if it existed
    println!("{}", set.remove(&1));
    println!("{}", set.remove(&99));

    // len and is_empty
    println!("Size: {}", set.len());
    println!("Empty: {}", set.is_empty());
    println!("{:?}", set);
}
*/

// ------------------------------------------

// Set Operations - The Mathematical Power
/*
use std::collections::HashSet;

fn main() {
    let set_a: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set_b: HashSet<i32> = [4, 5, 6, 7, 8].iter().cloned().collect();

    // UNION - all elements from both sets
    let union: HashSet<_> = set_a.union(&set_b).collect();
    println!("Union: {:?}", union);

    // INTERSETION - only element in BOTH sets
    let intersection: HashSet<_> = set_a.intersection(&set_b).collect();
    println!("Intersection: {:?}", intersection);

    // DIFFERENCE - elements in set_a but NOT in set_b
    let difference: HashSet<_> = set_a.difference(&set_b).collect();
    println!("Difference: {:?}", difference);

    // SYMMETRIC DIFFERENCE - elements in either but NOT both
    let sym_diff: HashSet<_> = set_a.symmetric_difference(&set_b).collect();
    println!("Symmetric difference: {:?}", sym_diff);

    // Subset check
    let small: HashSet<i32> = [1, 2].iter().cloned().collect();
    println!("Is subset: {}", small.is_subset(&set_a));
    println!("Is superset: {}", set_a.is_superset(&small));
}
*/

// -------------------------------------

// Iterating over HashSet

/*use std::collections::HashSet;

fn main() {
    let fruits: HashSet<&str> = ["apple", "banana", "mango"].iter().cloned().collect();

    for fruit in &fruits {
        println!("{}", fruit);
    }
}
*/

//--------------------------------------------------------------
// TODO: NOTE THAT:
/*
Vec<T>
→ Ordered list of items
→ You care about POSITION and ORDER
→ Duplicates are okay or expected
→ Examples: messages, scores, transaction history

HashMap<K, V>
→ You need to look things up BY KEY
→ Each item has an associated value
→ Examples: word counts, student grades, user profiles, caches

HashSet<T>
→ You only care if something EXISTS or NOT
→ Duplicates must be prevented
→ You need union/intersection operations
→ Examples: unique visitors, permissions, tags, already-seen items
 */

// ------------------------------------------------------

// All three together

use std::collections::{HashMap, HashSet};

fn analyze_text(text: &str) {
    let words: Vec<&str> = text.split_whitespace().collect();

    // Vec - ordered list of all words
    println!("All words: {:?}", words);
    println!("Total words: {}", words.len());

    // HashSet - unique words only
    let unique: HashSet<&str> = words.iter().cloned().collect();
    println!("Unique words: {}", unique.len());

    // HashMap - word frequency count
    let mut frequency: HashMap<&str, u32> = HashMap::new();
    for word in &words {
        *frequency.entry(word).or_insert(0) += 1;
    }

    println!("Frequencies:");
    for (word, count) in &frequency {
        println!("  {:<3}: {}", word, count);
    }
}

fn main() {
    analyze_text("the cat sat on the mat the cat");
}
