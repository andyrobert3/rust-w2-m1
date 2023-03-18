struct SavingsAccount {}
struct FixedDepositAccount {}
trait Interest {
    fn interest_rate(&self) -> u8 {
        // default interest rate for all
        5
    }
}

impl Interest for SavingsAccount {}
impl Interest for FixedDepositAccount {
    fn interest_rate(&self) -> u8 {
        // special interest rate for FD
        8
    }
}

fn main() {
    let john_account = SavingsAccount {};
    let sally_account = FixedDepositAccount {};
    println!("{}", john_account.interest_rate()); // prints 5
    println!("{}", sally_account.interest_rate()); // prints 8
}
