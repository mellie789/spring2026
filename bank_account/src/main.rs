/*
Create a new file named bank_account.rs in your src directory.

In bank_account.rs, define a BankAccount struct with the following:

A private field balance of type f64
A public method new(initial_balance: f64) -> BankAccount to create a new account
Public methods:
deposit(&mut self, amount: f64)
withdraw(&mut self, amount: f64)
balance(&self) -> f64
Implement the methods with the following rules:

deposit: Should increase the balance. Ignore the operation if the amount is negative.
withdraw: Should decrease the balance. If the amount is greater than the balance or negative, the balance should remain unchanged.
balance: Should return the current balance.
Write tests for your BankAccount struct. Include tests for:

Creating a new account
Depositing money
Withdrawing money
Checking the balance
Edge cases (e.g., depositing/withdrawing negative amounts, 
withdrawing more than the balance)
Update main.rs to demonstrate the use of your BankAccount struct.

Use assert_eq! to check if values are equal.
Remember to test both normal operations and edge cases.
For floating-point comparisons, you might want to use 
assert!((a - b).abs() < epsilon) where epsilon is a small 
number like 1e-10, to account for potential floating-point 
inaccuracies.

Implement and test an apply_interest method that 
increases the balance by a given interest rate.
When you're done, run your tests with `cargo test` 
and make sure they all pass!

*/

mod bank_account;
use bank_account::BankAccount;

fn main(){
    println!("Bank Account");
    println!("=============");

    //Create a new account
    let mut account = BankAccount::new(1000.0);
    println!("New Account Initial Balance: ${:.2}", account.balance());

    //Deposit money
    account.deposit(500.0);
    println!("After Deposit of $500.00: ${:.2}", account.balance());

    //Withdraw money
    account.withdraw(200.0);
    println!("After Withdrawal of $200.00: ${:.2}", account.balance());

    //Try to withdraw more than the balance
    account.withdraw(2000.0);
    println!("After Attempted Withdrawal of $2000.00 (more than balance): ${:.2}", account.balance());

    //Try to deposit a negative amount
    account.deposit(-100.0);
    println!("After Attempted Deposit of -$100.00 (negative amount): ${:.2}", account.balance());

    //Apply interest
    account.apply_interest(0.05);
    println!("After Applying 5% Interest: ${:.2}", account.balance());

    println!("\nEdge Cases:");
    println!("=============");

    let mut empty_account = BankAccount::new(0.0);
    println!("Empty Account Initial Balance: ${:.2}", empty_account.balance());

    empty_account.withdraw(50.0);
    println!("After Attempted Withdrawal of $50.00 from Empty Account: ${:.2}", empty_account.balance());

    empty_account.deposit(-30.0);
    println!("After Attempted Deposit of -$30.00 to Empty Account: ${:.2}", empty_account.balance());

    empty_account.apply_interest(0.05);
    println!("After Applying 5% Interest to Empty Account: ${:.2}", empty_account.balance());
}

