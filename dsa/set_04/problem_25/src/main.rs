#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
    length: u32,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: None,
            length: 0,
        }
    }

    fn push_front(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.length += 1;
    }

    fn push_back(&mut self, value: i32) {
        let new_node = Box::new(Node { value, next: None });

        // if list is empty - new node becomes head
        if self.head.is_none() {
            self.head = Some(new_node);
            self.length += 1;
            return;
        }

        // traverse to last node
        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.next.is_none() {
                // this is the last node - attach here
                node.next = Some(new_node);
                break;
            }
            current = &mut node.next;
        }
        self.length += 1;
    }

    fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.length -= 1;
            node.value
        })
    }

    fn print_list(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }

    fn length(&self) -> u32 {
        self.length
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    list.push_front(0);
    list.print_list();

    println!("Pop front: {}", list.pop_front().unwrap_or(-1));
    list.print_list();

    println!("Length: {}", list.length())
}
