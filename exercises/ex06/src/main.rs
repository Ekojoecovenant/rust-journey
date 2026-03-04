fn main() {
    let product: (&str, f64, i32, bool) = ("Rust Programming Book", 49.99, 150, true);

    println!("Name      : {}", product.0);
    println!("Price     : {}", product.1);
    println!("Stock     : {}", product.2);
    println!("Available : {}", product.3);

    let (name, price, stock, available) = product;
    let stock: f64 = stock.into(); // i dont know what the .into() is but it's what rust error stuff suggested for me so i added it
    let total_value = price * stock;

    println!("\n=== Product in Store ===");
    println!("Name        : {}", name);
    println!("Price       : {}", price);
    println!("Stock       : {}", stock);
    println!("Available   : {}", available);
    println!("Total Value : {}", total_value);
}
