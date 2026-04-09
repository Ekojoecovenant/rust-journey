trait Printable {
    fn print(&self);
}

trait Summarize {
    fn summary(&self) -> String;
}

struct Article {
    title: String,
    content: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Printable for Article {
    fn print(&self) {
        println!(
            "[Article] {} by {}\n{}",
            self.title, self.author, self.content
        );
    }
}

impl Printable for Tweet {
    fn print(&self) {
        println!("[Tweet] @{}: {}", self.username, self.content);
    }
}

impl Summarize for Article {
    fn summary(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

impl Summarize for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn display_summary(item: &(impl Printable + Summarize)) {
    item.print();
    println!("Summary: {}", item.summary());
}

fn display_summary2<T: Printable + Summarize>(item: &T) {
    item.print();
    println!("Summary: {}", item.summary());
}

fn main() {
    let article: Article = Article {
        title: "Test Article".to_string(),
        content: "This is a test article to test the project is it works".to_string(),
        author: "Cove".to_string(),
    };
    let tweet: Tweet = Tweet {
        username: "cove".to_string(),
        content: "This is my first tweet on twitter".to_string(),
    };

    display_summary(&article);
    println!("");
    display_summary2(&tweet);
}
