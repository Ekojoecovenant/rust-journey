struct Stack {
    data: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&i32> {
        self.data.last()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);
    println!("Push: 10, 20, 30");
    println!("{:?}\n", stack.data);

    println!("Peek: {}", stack.peek().unwrap_or(&-1));
    println!("Pop: {}", stack.pop().unwrap_or(-1));
    println!("Pop: {}", stack.pop().unwrap_or(-1));
    println!("Size: {}", stack.size());
    println!("is_empty: {}", stack.is_empty());
    println!("Pop: {}", stack.pop().unwrap_or(-1));
    println!("is_empty: {}", stack.is_empty());
}
