use std::collections::HashMap;

fn total_items(inventory: &HashMap<String, u32>) -> u32 {
    let mut total = 0u32;
    for v in inventory.values() {
        total += *v;
    }
    total
}

fn find_item(inventory: &HashMap<String, u32>, item: &str) -> Option<u32> {
    /*Method 1 */
    // for (k, v) in inventory {
    //     if *k == item.to_string() {
    //         return Some(*v);
    //     }
    // }
    // None

    /* Method 2 */
    if inventory.contains_key(item) {
        Some(inventory[item])
    } else {
        None
    }
}

fn low_stock(inventory: &HashMap<String, u32>, threshold: u32) -> Vec<String> {
    let mut low = Vec::new();

    for (k, v) in inventory {
        if *v < threshold {
            low.push(k.to_string());
        }
    }
    low
}

fn restock(inventory: &mut HashMap<String, u32>, item: &str, amount: u32) {
    let stock = inventory.entry(item.to_string()).or_insert(0u32);
    *stock += amount;
}

fn main() {
    let mut inventory: HashMap<String, u32> = HashMap::new();

    for (k, v) in [
        ("Apple", 50u32),
        ("Banana", 30),
        ("Mango", 20),
        ("Orange", 45),
        ("Pawpaw", 15),
    ] {
        inventory.insert(k.to_string(), v);
    }

    println!("=== Inventory System ===");
    println!("Total items : {}", total_items(&inventory));
    println!("Apple stock : {:?}", find_item(&inventory, "Apple"));
    println!("Mango stock : {:?}", find_item(&inventory, "Mango"));
    println!("Grape stock : {:?}", find_item(&inventory, "Grape"));

    println!("\nLow stock (threshold 25):");
    for v in low_stock(&inventory, 25) {
        println!("  {:<7} : {}", v, inventory[&v]);
    }

    restock(&mut inventory, "Coke", 10);
    println!("\nAfter restocking Coke by 10:");
    println!("  Coke: {}", inventory["Coke"]);

    restock(&mut inventory, "Mango", 40);
    println!("\nAfter restocking Mango by 40:");
    println!("  Mango: {}", inventory["Mango"]);
}
