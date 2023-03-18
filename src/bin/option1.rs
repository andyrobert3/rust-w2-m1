// this structure will contain Customer registration data for an email newsletter
// age is optional, and might not be provided, hence the use of Option
struct Customer {
    age: Option<i32>,   // age is optional!
    email: String,
 } 
 
 fn main() { 
    // Create customer data for Mark
    let mark = Customer {
        age: Some(22), 
        email: "mark@example.com".to_owned(),
    };
 
    // No age data was provided for Becky, so we use None
    let becky = Customer {
        age: None, 
        email: "becky@example.com".to_owned(),
    };
 
    // Do a match on Becky's age
    match becky.age {
        Some(age) => println!("Customer is {:?} years old", age),
        None => println!("Customer age was not provided"),
    }
 }
 
 
 
 
 