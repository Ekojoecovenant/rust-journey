struct Queue {
    data: Vec<i32>,
}

impl Queue {
    fn new() -> Self {
        Queue { data: Vec::new() }
    }

    fn enqueue(&mut self, value: i32) {
        self.data.push(value);
    }

    fn dequeue(&mut self) -> Option<i32> {
        // if self.data.get(0).is_some() { (my first solution but i wanted to be efficiently close to O(n) or O(1) so i used the one below)
        if !self.is_empty() {
            return Some(self.data.remove(0));
        }
        None
    }

    fn front(&self) -> Option<&i32> {
        self.data.first()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    let mut queue = Queue::new();

    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);
    println!("Enqueue: 10, 20, 30");
    println!("{:?}\n", queue.data);

    println!("Front: {}", queue.front().unwrap_or(&-1));
    println!("Dequeue: {}", queue.dequeue().unwrap_or(-1));
    println!("Dequeue: {}", queue.dequeue().unwrap_or(-1));
    println!("Size: {}", queue.size());
    println!("is_empty: {}", queue.is_empty());
    println!("Dequeue: {}", queue.dequeue().unwrap_or(-1));
    println!("is_empty: {}", queue.is_empty());
}
