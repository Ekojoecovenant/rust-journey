trait Animal {
    fn name(&self) -> String;
    fn sound(&self) -> String;
}

struct Dog {
    name: String,
}
struct Cat {
    name: String,
}
struct Cow {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn sound(&self) -> String {
        String::from("Woof")
    }
}

impl Animal for Cat {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn sound(&self) -> String {
        String::from("Meow")
    }
}

impl Animal for Cow {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn sound(&self) -> String {
        String::from("Moo")
    }
}

fn introduce(animal: &impl Animal) {
    println!("Hi, I'm a {} and I say {}!", animal.name(), animal.sound());
}

fn main() {
    let dog = Dog {
        name: String::from("Rex"),
    };
    let cat = Cat {
        name: String::from("Whiskers"),
    };
    let cow = Cow {
        name: String::from("Bessie"),
    };

    introduce(&dog);
    introduce(&cat);
    introduce(&cow);
}
