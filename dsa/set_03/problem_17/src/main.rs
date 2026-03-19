fn push(stack: &mut ([i32; 10], usize), val: i32) -> bool {
    if is_full(stack) {
        false
    } else {
        stack.0[stack.1] = val;
        stack.1 += 1;
        true
    }
}

fn pop(stack: &mut ([i32; 10], usize)) -> i32 {
    if is_empty(stack) {
        -1
    } else {
        let last_num = stack.0[stack.1 - 1];
        stack.0[stack.1 - 1] = 0;
        stack.1 -= 1;
        last_num
    }
}

// not exactly sure what peek is supposed to do..but i'll assume it's supposed to return the last index
fn peek(stack: &([i32; 10], usize)) -> i32 {
    if is_empty(stack) {
        -1
    } else {
        stack.0[stack.1 - 1]
    }
}

fn is_empty(stack: &([i32; 10], usize)) -> bool {
    stack.1 == 0
}

fn is_full(stack: &([i32; 10], usize)) -> bool {
    stack.1 >= 10
}

fn print_stack(stack: &([i32; 10], usize)) {
    // not full sure what how the "&stack.0[..stack.1]" behind the scenes...but..
    // it'll slice the stack.0 array and return the number of elements based on the number in stack.1
    // then i print it
    let arr = &stack.0[..stack.1];
    // hold on...i think i get it...so it's like a mini loop or so
    // it'll count from 0 till the value of stack.1 and stop then return what was looped to arr as an array
    // damnnnn....that's crazy and beautiful at the same time....i think im in love with a programming language
    println!("Stack: {:?}", arr);
}

fn main() {
    let mut arr: ([i32; 10], usize) = ([0; 10], 0);

    println!("=== Stck Operations ===");
    println!("Empty? {}", is_empty(&arr));
    println!("Push 10: {}", push(&mut arr, 10));
    println!("Push 20: {}", push(&mut arr, 20));
    println!("Push 30: {}", push(&mut arr, 30));
    print_stack(&mut arr);
    println!("Peek: {}", peek(&arr));
    println!("Pop: {}", pop(&mut arr));
    println!("Pop: {}", pop(&mut arr));
    print_stack(&mut arr);
    println!("Empty? {}", is_empty(&arr));
}
