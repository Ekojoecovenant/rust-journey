/* ================ OWNERSHIP WITH FUNCTIONS =========== */

// fn take_string(s: String) {
//     // s receives ownership
//     println!("{}", s);
// } // s goes out of scope — String is DROPPED here

// fn take_number(n: i32) {
//     // n receives a COPY
//     println!("{}", n);
// } // n goes out of scope — copy is dropped — original unaffected

// fn main() {
//     let s1 = String::from("hello");
//     take_string(s1); // s1 MOVES into function
//     // println!("{}", s1);      // ❌ s1 is gone

//     let x = 5;
//     take_number(x); // x is COPIED into function
//     println!("{}", x); // ✅ x still valid — it was copied
// }

/* ==========================||============================ */

/* ================== REFERENCES AND BORROWING =============== */

fn calculate_length(s: &String) -> usize {
    //                     ^ & means reference — borrowing, not owning
    s.len()
} // s (the reference) goes out of scope
// but since it doesn't OWN the String — nothing is dropped!

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    //                              ^ & means "lend this, don't give it away"

    println!("Length of {} is {}", s1, len); // ✅ s1 still valid!
}

/* ========================|===||===|========================= */
