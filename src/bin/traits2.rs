struct SavingsAccount {
    name: String,
}
struct FixedDeposit {
    name: String,
}
struct HomeLoan {
    name: String,
}

trait Withdraw {}

impl Withdraw for SavingsAccount {}
impl Withdraw for FixedDeposit {}

// only accepts a type (generic) that implements Withdraw trait
fn withdraw_balance<T: Withdraw>(account: T, amount: f32) {
    println!("Withdraw {}", amount);
}

fn main() {
    let savings_account = SavingsAccount {
        name: String::from("John"),
    };
    let fixed_deposit = FixedDeposit {
        name: String::from("Sally"),
    };
    let home_loan = HomeLoan {
        name: String::from("George"),
    };
    withdraw_balance(savings_account, 50.0);
    withdraw_balance(fixed_deposit, 50.0);
    withdraw_balance(home_loan, 50.0); // error - the trait `Withdraw` is not implemented for `HomeLoan`
}
