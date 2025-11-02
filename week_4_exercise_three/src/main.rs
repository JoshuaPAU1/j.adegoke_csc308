struct BankAccount{
    account_number: String,
    balance: f64,
}

impl BankAccount {
    fn new(account_number: String, initial_balance: f64) -> Self {
        BankAccount {
            account_number,
            balance: initial_balance,
        }
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("Deposited: ${:.2}", amount);
        } else {
            println!("Deposit amount must be positive.");
        }
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            println!("Withdrew: ${:.2}", amount);
        } else if amount > self.balance {
            println!("Insufficient funds for withdrawal.");
        } else {
            println!("Withdrawal amount must be positive.");
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}



fn main() {
    let mut account = BankAccount::new("123456789".to_string(), 1000.0);
    account.deposit(500.0);
    account.withdraw(200.0);
    println!("Current balance: ${:.2}", account.get_balance());
}
