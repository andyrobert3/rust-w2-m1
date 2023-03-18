fn main() {
    let max_transactions = Some(10);

    // Match method
    match max_transactions {
        Some(max) => println!("The maximum transactions is {}", max),
        _ => (), // this line is required for Match exhaustive rule
    }

    // If Let Method - does not need to be exhaustive like Match, write less code
    if let Some(max) = max_transactions {
        // if max_transaction is of Some(x)
        println!("The maximum transactions is {}", max);
    } // use an else here if required
}
