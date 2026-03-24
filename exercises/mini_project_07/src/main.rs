#[derive(Debug)]
enum CalcError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
    InvalidOperation,
}

struct Calculator {
    history: [f64; 20],
    history_count: usize,
    last_result: Option<f64>,
}

impl Calculator {
    fn new() -> Self {
        Calculator {
            history: [0f64; 20],
            history_count: 0,
            last_result: None,
        }
    }

    fn add(&mut self, a: f64, b: f64) -> Result<f64, CalcError> {
        let result = a + b;
        self.record(result)
    }

    fn subtract(&mut self, a: f64, b: f64) -> Result<f64, CalcError> {
        let result = a - b;
        self.record(result)
    }

    fn multiply(&mut self, a: f64, b: f64) -> Result<f64, CalcError> {
        let result = a * b;
        self.record(result)
    }

    fn divide(&mut self, a: f64, b: f64) -> Result<f64, CalcError> {
        if b == 0.0 {
            return Err(CalcError::DivisionByZero);
        }
        let result = a / b;
        self.record(result)
    }

    fn sqrt(&mut self, a: f64) -> Result<f64, CalcError> {
        if a < 0.0 {
            return Err(CalcError::NegativeSquareRoot);
        }
        let result = a.sqrt();
        self.record(result)
    }

    fn power(&mut self, base: f64, exp: u32) -> Result<f64, CalcError> {
        let result = base.powf(exp.into());
        self.record(result)
    }

    fn last_result(&self) -> Option<f64> {
        self.last_result
    }

    fn print_history(&self) {
        println!("=== Calculation History ===");
        let history = &self.history[..(self.history_count as usize)];
        let mut count = 1;

        for calc in history {
            println!("{:<2} {}", format!("{}.", count), {
                (calc * 100.0).round() / 100.0
            });
            count += 1;
        }
        println!("[failed operations not recorded]");
    }

    fn record(&mut self, result: f64) -> Result<f64, CalcError> {
        self.history[self.history_count] = result;
        self.history_count += 1;
        self.last_result = Some(result);
        Ok(result)
    }
}

fn main() {
    // println!("║  2 ^ 10     = {:<22}║")
    let mut calculator1 = Calculator::new();
    println!("╔═══════════════════════════════════════╗");
    println!("║      CALCULATOR WITH ERROR HANDLING   ║");
    println!("╠═══════════════════════════════════════╣");
    println!("║  10 + 3     = {:<22} ║", {
        match calculator1.add(10.0, 3.0) {
            Ok(result) => format!("✅ {}", result),
            Err(error) => format!("❌ {:?}", error),
        }
    });
    println!("║  10 - 3     = {:<22} ║", {
        match calculator1.subtract(10.0, 3.0) {
            Ok(result) => format!("✅ {}", result),
            Err(error) => format!("❌ {:?}", error),
        }
    });
    println!("║  10 * 3     = {:<22} ║", {
        match calculator1.multiply(10.0, 3.0) {
            Ok(result) => format!("✅ {}", result),
            Err(error) => format!("❌ {:?}", error),
        }
    });
    println!("║  10 / 3     = {:<22} ║", {
        match calculator1.divide(10.0, 3.0) {
            Ok(result) => format!("✅ {:.2}", result),
            Err(error) => format!("❌ {:?}", error),
        }
    });
    println!("║  10 / 0     = {:<22} ║", {
        match calculator1.divide(10.0, 0.0) {
            Ok(result) => format!("✅ {}", result),
            Err(error) => format!("❌ {:?}", error),
        }
    });
    println!("║  √16        = {:<22} ║", {
        match calculator1.sqrt(16.0) {
            Ok(result) => format!("✅ {}", result),
            Err(error) => format!("❌ {:?}", error),
        }
    });
    println!("║  √-4        = {:<22} ║", {
        match calculator1.sqrt(-4.0) {
            Ok(result) => format!("✅ {}", result),
            Err(error) => format!("❌ {:?}", error),
        }
    });
    println!("║  2 ^ 10     = {:<22} ║", {
        match calculator1.power(2.0, 10) {
            Ok(result) => format!("✅ {}", result),
            Err(error) => format!("❌ {:?}", error),
        }
    });
    println!("╚═══════════════════════════════════════╝\n");

    println!("Last result: {:?}\n", calculator1.last_result());

    calculator1.print_history();
}
