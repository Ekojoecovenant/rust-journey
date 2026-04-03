use std::collections::{HashMap, HashSet};

struct Contact {
    name: String,
    phone: String,
    email: String,
}

struct ContactBook {
    contacts: Vec<Contact>,
    phone_index: HashMap<String, usize>,
    favorites: HashSet<String>,
}

impl ContactBook {
    fn new() -> Self {
        ContactBook {
            contacts: Vec::new(),
            phone_index: HashMap::new(),
            favorites: HashSet::new(),
        }
    }

    fn add_contact(&mut self, name: &str, phone: &str, email: &str) {
        self.contacts.push(Contact {
            name: name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
        });
        self.phone_index
            .insert(phone.to_string(), self.contacts.len() - 1);

        println!("Added: {}", name);
    }

    fn find_by_phone(&self, phone: &str) -> Option<&Contact> {
        if self.phone_index.contains_key(phone) {
            let contact = self.phone_index.get(phone)?;
            Some(&self.contacts[*contact])
        } else {
            None
        }
    }

    fn add_favorite(&mut self, name: &str) -> bool {
        self.favorites.insert(name.to_string())
    }

    fn remove_favorite(&mut self, name: &str) -> bool {
        self.favorites.remove(name)
    }

    fn list_favorites(&self) {
        println!("\nFavorites:");
        for name in &self.favorites {
            println!("  ⭐ {}", *name);
        }
    }

    fn list_all(&self) {
        println!("\nAll Contacts:");
        let mut index = 0;
        for contact in &self.contacts {
            index += 1;
            println!(
                "  {:>3} {:<9} | {:<12} | {}",
                format!("{}.", index),
                contact.name,
                contact.phone,
                contact.email,
            );
        }
    }

    fn total_contacts(&self) -> usize {
        self.contacts.len()
    }
}

fn main() {
    println!("=== Contact Book ===");

    let mut contact_book1 = ContactBook::new();

    // add contacts
    contact_book1.add_contact("Cove", "+234-801-0000", "cove@email.com");
    contact_book1.add_contact("Ada", "+234-802-0000", "ada@email.com");
    contact_book1.add_contact("Turing", "+234-803-0000", "turing@email.com");

    // list all contacts
    contact_book1.list_all();

    // find a contact
    println!("\nFind +234-802-0000 : {}", {
        match contact_book1.find_by_phone("+234-802-0000") {
            None => "Not found".to_string(),
            Some(contact) => format!("{} | {}", contact.name, contact.email),
        }
    });

    // add and list favorites
    contact_book1.add_favorite("Cove");
    contact_book1.add_favorite("Turing");
    contact_book1.list_favorites();

    // remove and list favorites
    contact_book1.remove_favorite("Turing");
    contact_book1.list_favorites();

    // total number of contact
    println!("\nNumber of contacts: {}", contact_book1.total_contacts());
}
