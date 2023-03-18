use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new(); // {}
    users.insert("Jack", 21); // {Jack: 21}
    users.insert("Jill", 18); // {Jack: 21, Jill: 18}
    users.insert("Adam", 15); // {Jack: 21, Jill: 18, Adam: 15}
    users.remove("Jill"); // {Jack: 21, Adam: 15}
    users.insert("Jack", 50); // update existing entry

    match users.get("Adam") {
        //.get() returns an Option type!!
        Some(age) => println!("Adam's age: {:?}", age), // prints 15
        None => println!("Not found"),
    }
    match users.get("Jill") {
        //.get() returns an Option type!!
        Some(age) => println!("Jill's age: {:?}", age), // does this print?
        None => println!("Not found"),                  // does this print?
    }
    match users.get("Jack") {
        //.get() returns an Option type!!
        Some(age) => println!("Jack's age: {:?}", age), // what does this print?
        None => println!("Not found"),
    }
}
