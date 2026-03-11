fn main() {
    // Bug 1:
    println!("--- Fix 1 ---");
    // let mut s = String::from("hello");
    let s = String::from("hello");
    let r1 = &s;
    // let r2 = &mut s; // Error!
    // the bug was a violation of the reference rule that states that a variable cannot have a mutable and immutable reference
    // so to fix it...i have to turn the mutable reference to immutable reference
    // since it can have multiple immutable references and also because there is no need for a mutable reference as the variable is only being read
    // and i also made the mutable variable to be immutable
    let r2 = &s;
    println!("{} {}", r1, r2);

    // Bug 2:
    println!("\n--- Fix 2 ---");
    let s1 = String::from("hello");
    // let s2 = s1; // Error!
    // this is the cause of the error as the ownership has moved to s2 and completely discarded s1
    // therefore there are three solution to this
    // 1: to make s2 a reference to s1
    // 2: to clone s1 into s2
    // 3: print s2(valid) instead of s1(invalid)
    // since the value of s1 is not too large, cloning or referencing might fit in as a perfect solution
    // however, the value of s2 was not used, therefore we'll move ownership to s2 and print s2 instead of s1
    let s2 = s1;
    println!("{}", s2);

    // Bug 3:
    println!("\n--- Fix 3 ---");
    let s = get_reference();
    println!("Reference : {}", s);

    // Bug 4:
    println!("\n--- Fix 4 ---");
    let mut v = String::from("hello");
    // let r = &v;
    // im still shaky on how this works...but i was able to deduce the following
    // - r = immutable reference, while v (using .push_str) became a mutable borrow thereby conflicting
    // - v wasnt seen a mutable reference until it used .push_str
    // - if i make r mutable, it'll conflict because of a rule that states that you cant have 2 mutable references
    // i can deduce two solutions for this
    // - swap the lines of code for r initialization and the v.push_str stuff
    // - make r a mutable reference of v and use r to push the string (instead of v)
    v.push_str(" world");
    let r = &v;
    println!("{}", r);
}

fn get_reference() -> String {
    let s = String::from("temporary");
    s
    // this is a dangling reference.. there's a rule that you cannot return the reference to a memory that no longer exist
    // why does "s" not exist when it's literally created?
    // it's simply because by the time the reference is returned the variable would have died due to it leaving the scope upon which it was created
    // to solve this, i removed the reference from the return type and value, so the ownership of the value will be moved
}
