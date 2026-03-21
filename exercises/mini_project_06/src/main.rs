struct Transaction {
    amount: f64,
    transaction_type: &'static str, // "deposit" or "withdrawal"
    balance_after: f64,
}

struct BankAccount {
    owner: String,
    account_number: u32,
    balance: f64,
    transactions: [Transaction; 10], // max 10 transactions
    transaction_count: usize,
}

impl Transaction {
    fn new(amount: f64, transaction_type: &'static str, balance_after: f64) -> Self {
        Transaction {
            amount,
            transaction_type,
            balance_after,
        }
    }

    fn default() -> Self {
        Transaction {
            amount: 0.0,
            transaction_type: "none",
            balance_after: 0.0,
        }
    }

    fn describe(&self) {
        println!(
            "{:<11} ${:<8.2} -> balance: ${:.2}",
            self.transaction_type, self.amount, self.balance_after
        );
    }
}

impl BankAccount {
    fn new(owner: &str, account_number: u32, initial_balance: f64) -> Self {
        BankAccount {
            owner: String::from(owner),
            account_number,
            balance: initial_balance,
            transactions: [
                Transaction::default(),
                Transaction::default(),
                Transaction::default(),
                Transaction::default(),
                Transaction::default(),
                Transaction::default(),
                Transaction::default(),
                Transaction::default(),
                Transaction::default(),
                Transaction::default(),
            ],
            transaction_count: 0,
        }
    }

    fn deposit(&mut self, amount: f64) -> bool {
        if amount <= 0.0 {
            return false;
        }

        if self.transaction_count() < 10 {
            self.balance += amount;
            self.transactions[self.transaction_count()] =
                Transaction::new(amount, "deposit", self.get_balance());
            self.transaction_count += 1;
        }

        true
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if self.get_balance() < amount {
            return false;
        }

        if self.transaction_count() < 10 {
            self.balance -= amount;
            self.transactions[self.transaction_count()] =
                Transaction::new(amount, "withdrawal", self.get_balance());
            self.transaction_count += 1;
        }

        true
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }

    fn print_statement(&self) {
        println!("=== Account Statement ===");
        println!("Owner   : {}", self.owner);
        println!("Account : {}", self.account_number);
        println!("Balance : ${:.2}", self.get_balance());
    }

    fn transaction_count(&self) -> usize {
        self.transaction_count
    }

    fn is_overdrawn(&self) -> bool {
        self.get_balance() < 0.0
    }
}

fn print_transaction(success: bool, operation: &str, amount: f64, balance: f64) {
    print!("{:<9} ${:<8.2}", operation, amount);
    if success {
        println!(" ✅ New balance: ${:.2}", balance)
    } else {
        println!(" ❌ Insufficient funds");
    }
}

fn main() {
    let mut account1 = BankAccount::new("Cove", 10042024, 1000.00);

    println!("╔══════════════════════════════════════╗");
    println!("║         BANK ACCOUNT SYSTEM          ║");
    println!("╠══════════════════════════════════════╣");
    println!("║  Owner   : {:<11}               ║", account1.owner);
    println!(
        "║  Account : {:<11}               ║",
        account1.account_number
    );
    println!("║  Balance : ${:<10.2}               ║", account1.balance);
    println!("╚══════════════════════════════════════╝");

    println!("");

    let success = account1.deposit(500.0);
    print_transaction(success, "deposit", 500.0, account1.get_balance());

    let success = account1.deposit(250.0);
    print_transaction(success, "deposit", 250.0, account1.get_balance());

    let success = account1.withdraw(200.0);
    print_transaction(success, "withdrawal", 200.0, account1.get_balance());

    let success = account1.withdraw(100.0);
    print_transaction(success, "withdrawal", 100.0, account1.get_balance());

    let success = account1.withdraw(2000.0);
    print_transaction(success, "withdrawal", 2000.0, account1.get_balance());

    println!("");
    account1.print_statement();

    println!("\nTransactions:");
    for i in 0..account1.transaction_count() {
        print!("  {}. ", i + 1);
        account1.transactions[i].describe();
    }

    println!("\nTotal transactions: {}", account1.transaction_count());
    println!("Overdrawn: {}", account1.is_overdrawn());
}
