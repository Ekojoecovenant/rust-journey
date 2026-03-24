struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }

    fn greet(&self) {
        println!("Hi I'm {} and I'm {} years old", self.name, self.age);
    }
}

fn main() {
    // let person1 = Person::new("Cove".to_string(), 19);
    // person1.greet();

    // success
    match divide(10.0, 2.0) {
        Ok(result) => println!("✅ Success : {:.2}", result),
        Err(e) => println!("❌ Error: {}", e),
    }

    // fail
    match divide(10.0, 0.0) {
        Ok(result) => println!("✅ Success : {:.2}", result),
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn divide(num1: f64, num2: f64) -> Result<f64, String> {
    if num2 == 0.0 {
        return Err(String::from("cannot be divided by zero"));
    }
    Ok(num1 / num2)
}
