struct Book {
    title: String,
    author: String,
    pages: u32,
    price: f64,
    is_available: bool,
}

impl Book {
    fn new(title: &str, auth: &str, pages: u32, price: f64) -> Self {
        Book {
            title: String::from(title),
            author: String::from(auth),
            pages,
            price,
            is_available: true,
        }
    }

    fn describe(&self) {
        println!("=== Library System ===");
        println!("Title     : {}", self.title);
        println!("Author    : {}", self.author);
        println!("Pages     : {}", self.pages);
        println!("Price     : ${}", self.price);
        println!("Available : {}", self.is_available);
        println!("Long book : {}", self.is_long_book());
    }

    fn discount(&mut self, percent: f64) {
        let discount = self.price * percent / 100.0;
        self.price = ((self.price - discount) * 100.0).round() / 100.0;
    }

    fn checkout(&mut self) {
        self.is_available = false;
    }

    fn return_book(&mut self) {
        self.is_available = true;
    }

    fn is_long_book(&self) -> bool {
        self.pages > 300
    }
}

fn main() {
    let mut book1 = Book::new("The Rust Programming Language", "Steve Klabnik", 526, 39.99);

    book1.describe();

    println!("\nAfter 20% discount:");
    book1.discount(20.0);
    println!("Price     : ${}", book1.price);

    println!("\nAfter checkout:");
    book1.checkout();
    println!("Available : {}", book1.is_available);

    println!("\nAfter returning book:");
    book1.return_book();
    println!("Available : {}", book1.is_available);
}
