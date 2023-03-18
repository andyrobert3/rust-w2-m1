enum PaymentType {
    CreditCard,
    Cash,
    DigitalToken,
}

fn main() {
    let credit_card = PaymentType::CreditCard;

    if let PaymentType::CreditCard = credit_card {
        println!("Credit Card!");
    } // insert an else here if required
}
