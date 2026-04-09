trait Notification {
    fn title(&self) -> String;

    fn message(&self) -> String;

    fn notify(&self) {
        println!("[NOTIFICATION]");
        println!("Title: {}\nMessage: {}", self.title(), self.message());
    }

    fn priority(&self) -> String {
        String::from("Normal")
    }
}

struct EmailNotification {
    title: String,
    message: String,
}

struct SmsNotification {
    title: String,
    message: String,
    priority: String,
}

impl Notification for EmailNotification {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn message(&self) -> String {
        self.message.clone()
    }
}

impl Notification for SmsNotification {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn message(&self) -> String {
        self.message.clone()
    }

    fn priority(&self) -> String {
        self.priority.clone()
    }
}

fn main() {
    let email = EmailNotification {
        title: "New Login".to_string(),
        message: "Login successful".to_string(),
    };
    let sms = SmsNotification {
        title: "Verify Number".to_string(),
        message: "This is the otp to verify your phone number".to_string(),
        priority: "High".to_string(),
    };

    email.notify();
    println!("");
    sms.notify();

    println!("\n[.priority()]");
    println!("{}", email.priority());
    println!("{}", sms.priority());
}
