struct Character {
    name: String,
    health: u32,
    attack: u32,
    defense: u32,
    level: u32,
    class: String,
}

impl Character {
    fn new(name: &str) -> Self {
        Character {
            name: String::from(name),
            health: 100,
            attack: 10,
            defense: 5,
            level: 1,
            class: String::from("Warrior"),
        }
    }

    fn health(mut self, hp: u32) -> Self {
        self.health = hp;
        self
    }

    fn attack(mut self, atk: u32) -> Self {
        self.attack = atk;
        self
    }

    fn defense(mut self, def: u32) -> Self {
        self.defense = def;
        self
    }

    fn level(mut self, lvl: u32) -> Self {
        self.level = lvl;
        self
    }

    fn class(mut self, cls: &str) -> Self {
        self.class = String::from(cls);
        self
    }

    fn build(self) -> Self {
        self
    }

    fn stats(&self) {
        println!("Name     : {}", self.name);
        println!("Class    : {}", self.class);
        println!("Level    : {}", self.level);
        println!("Health   : {}", self.health);
        println!("Attack   : {}", self.attack);
        println!("Defense  : {}", self.defense);
        println!("Powerful : {}", self.is_powerful());
    }

    fn is_powerful(&self) -> bool {
        (self.attack + self.defense) > 100
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.attack += 5;
        self.defense += 3;
    }
}

fn main() {
    let character1 = Character::new("Goblin");
    let mut character2 = Character::new("Cove")
        .class("mage")
        .level(5)
        .health(200)
        .attack(75)
        .defense(40)
        .build();
    let character3 = Character::new("Third").class("Archer").attack(40).build();

    println!("=== Character Creation ===");

    println!("\n--- Basic Character ---");
    character1.stats();

    println!("\n--- Custom Character ---");
    character2.stats();

    character2.level_up();
    println!("\n--- After Level Up ---");
    println!("Name     : {}", character2.name);
    println!("Level    : {}", character2.level);
    println!("Attack   : {}", character2.attack);
    println!("Defense  : {}", character2.defense);

    println!("");
    character3.stats();
}
