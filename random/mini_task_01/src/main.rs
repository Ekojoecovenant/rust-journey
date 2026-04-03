// struct Person {
//     name: String,
//     age: u8,
// }

// impl Person {
//     fn new(name: String, age: u8) -> Self {
//         Person { name, age }
//     }

//     fn greet(&self) {
//         println!("Hi I'm {} and I'm {} years old", self.name, self.age);
//     }
// }

// fn main() {
//     // let person1 = Person::new("Cove".to_string(), 19);
//     // person1.greet();

//     // success
//     match divide(10.0, 2.0) {
//         Ok(result) => println!("✅ Success : {:.2}", result),
//         Err(e) => println!("❌ Error: {}", e),
//     }

//     // fail
//     match divide(10.0, 0.0) {
//         Ok(result) => println!("✅ Success : {:.2}", result),
//         Err(e) => println!("❌ Error: {}", e),
//     }
// }

// fn divide(num1: f64, num2: f64) -> Result<f64, String> {
//     if num2 == 0.0 {
//         return Err(String::from("cannot be divided by zero"));
//     }
//     Ok(num1 / num2)
// }

trait Describable {
    fn describe(&self) -> String;

    fn print_description(&self) {
        println!("{}", self.describe());
    }
}

struct Car {
    brand: String,
    year: u16,
}

struct Phone {
    brand: String,
    model: String,
}

impl Describable for Car {
    fn describe(&self) -> String {
        format!("Car: {} ({})", self.brand, self.year)
    }
}

impl Describable for Phone {
    fn describe(&self) -> String {
        format!("Phone: {} - {}", self.brand, self.model)
    }
}

fn main() {
    let car1 = Car {
        brand: "Toyota".to_string(),
        year: 2019,
    };

    let phone1 = Phone {
        brand: "Apple".to_string(),
        model: "iPhone 15".to_string(),
    };

    car1.print_description();
    phone1.print_description();
}
