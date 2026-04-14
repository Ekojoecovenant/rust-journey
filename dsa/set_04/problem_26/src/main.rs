struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

struct BST {
    root: Option<Box<TreeNode>>,
}

impl BST {
    fn new() -> Self {
        BST { root: None }
    }

    fn insert(&mut self, value: i32) {}
}

fn main() {
    println!("Hello, world!");
}
