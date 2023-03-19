use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new(); // {}
    users.insert("Jack", 21); // {Jack: 21}
    users.insert("Jill", 18); // {Jack: 21, Jill: 18}
    users.insert("Adam", 15); // {Jack: 21, Jill: 18, Adam: 15}

    for person in users.keys() {
        // .keys() returns keys only
        println!("person = {}", person); // prints Jack, Jill, Adam
        match users.get(person) {
            Some(x) => println!("{}", x),
            None => println!("")
        }
    }
}
