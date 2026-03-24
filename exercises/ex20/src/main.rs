enum Notification {
    Email {
        from: String,
        subject: String,
        body: String,
    },
    SMS {
        from: String,
        message: String,
    },
    Push {
        title: String,
        message: String,
    },
    InApp {
        message: String,
        priority: u32,
    },
}

impl Notification {
    fn display(&self) {
        println!("===== Notification =====");
        match self {
            Self::Email {
                from,
                subject,
                body,
            } => {
                println!("Type     : Email");
                println!("From     : {}", from);
                println!("Subject  : {}", subject);
                println!("Body     : {}", body);
            }
            Self::SMS { from, message } => {
                println!("Type     : SMS");
                println!("From     : {}", from);
                println!("Message  : {}", message);
            }
            Self::Push { title, message } => {
                println!("Type     : Push");
                println!("Title    : {}", title);
                println!("Message  : {}", message);
            }
            Self::InApp { message, priority } => {
                println!("Type     : InApp");
                println!("Message  : {}", message);
                println!("Priority : {}", priority);
            }
        }
    }

    fn is_urgent(&self) -> bool {
        match self {
            Self::Email { subject, .. } => subject.to_uppercase().contains("URGENT"),
            Self::SMS { .. } => true,
            Self::Push { .. } => true,
            Self::InApp { priority, .. } => *priority >= 8,
        }
    }

    fn sender(&self) -> &str {
        match self {
            Self::Email { from, .. } => from,
            Self::SMS { from, .. } => from,
            Self::Push { .. } => "System",
            Self::InApp { .. } => "App",
        }
    }
}

fn main() {
    let notification1 = Notification::Email {
        from: "test@gmail.com".to_string(),
        subject: "test".to_string(),
        body: "This is an Email test notification".to_string(),
    };
    let notification2 = Notification::Push {
        title: "Test".to_string(),
        message: "This is a push notification".to_string(),
    };
    let notification3 = Notification::InApp {
        message: "InApp Test completed".to_string(),
        priority: 7,
    };
    let notification4 = Notification::SMS {
        from: "Cove".to_string(),
        message: "User1".to_string(),
    };

    println!("Notif 1 (Is Urgent): {}", notification1.is_urgent());
    println!("Notif 2 (Is Urgent): {}", notification2.is_urgent());
    println!("Notif 3 (Is Urgent): {}", notification3.is_urgent());
    println!("Notif 4 (Is Urgent): {}\n", notification4.is_urgent());

    println!("Notif 1 (sender): {}", notification1.sender());
    println!("Notif 2 (sender): {}", notification2.sender());
    println!("Notif 3 (sender): {}", notification3.sender());
    println!("Notif 4 (sender): {}\n", notification4.sender());

    notification1.display();
    println!("");
    notification2.display();
    println!("");
    notification3.display();
    println!("");
    notification4.display();
}
