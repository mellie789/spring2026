#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount { 
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }

    pub fn apply_interest(&mut self, rate: f64) {
        // Implement this method
        if rate > 0.0 {
            self.balance += self.balance * rate;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account = BankAccount::new(100.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);

        let account = BankAccount::new(0.0);
        assert!((account.balance() - 0.0).abs() < EPSILON);

        let account = BankAccount::new(-50.0);
        assert!((account.balance() - (-50.0)).abs() < EPSILON);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(100.0);

        //Normal Deposit
        account.deposit(50.0);
        assert!((account.balance() - 150.0).abs() < EPSILON);

        //Multiple Deposits
        account.deposit(25.0);
        account.deposit(25.0);
        assert!((account.balance() - 200.0).abs() < EPSILON);

        //Deposit zero
        account.deposit(0.0);
        assert!((account.balance() - 200.0).abs() < EPSILON);

        //Deposit negative amount
        account.deposit(-50.0);
        assert!((account.balance() - 200.0).abs() < EPSILON);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(100.0);

        //Normal Withdraw
        account.withdraw(40.0);
        assert!((account.balance() - 60.0).abs() < EPSILON);

        //Multiple Withdraws
        account.withdraw(10.0);
        account.withdraw(20.0);
        assert!((account.balance() - 30.0).abs() < EPSILON);

        //Withdraw zero
        account.withdraw(0.0);
        assert!((account.balance() - 30.0).abs() < EPSILON);

        //Withdraw negative amount
        account.withdraw(-10.0);
        assert!((account.balance() - 30.0).abs() < EPSILON);

        //Withdraw more than balance
        account.withdraw(100.0);        
        assert!((account.balance() - 30.0).abs() < EPSILON);

        //Withdraw exact balance
        account.withdraw(30.0);
        assert!((account.balance() - 0.0).abs() < EPSILON);
    }

    // Add more tests here

    #[test]
    fn test_balance(){
        let mut account = BankAccount::new(100.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);

        account.deposit(50.0);
        assert!((account.balance() - 150.0).abs() < EPSILON);

        account.withdraw(30.0);
        assert!((account.balance() - 120.0).abs() < EPSILON);
    }

    #[test]
    fn test_edge_cases(){
        let mut account = BankAccount::new(100.0);

        //Try to withdraw from zero balance
        account.withdraw(100.0);
        assert!((account.balance() - 0.0).abs() < EPSILON);

        //Deposit negative amount to zero balance
        account.deposit(-50.0);
        assert!((account.balance() - 0.0).abs() < EPSILON);

        //Deposit and then try to withdraw more than balance
        account.deposit(100.0);
        account.withdraw(150.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }

    fn test_apply_interest() {
        let mut account = BankAccount::new(100.0);

        //Normal Interest
        account.apply_interest(0.05);
        assert!((account.balance() - 105.0).abs() < EPSILON);

        //Apply interest again
        account.apply_interest(0.10);
        assert!((account.balance() - 115.5).abs() < EPSILON);

        //Apply zero interest
        account.apply_interest(0.0);
        assert!((account.balance() - 115.5).abs() < EPSILON);

        //Apply negative interest
        account.apply_interest(-0.10);
        assert!((account.balance() - 115.5).abs() < EPSILON);

        //Interest on zero balance
        let mut zero_account = BankAccount::new(0.0);
        zero_account.apply_interest(0.05);
        assert!((zero_account.balance() - 0.0).abs() < EPSILON);
    }
}