use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new(); // {}
    users.insert("Jack", 21); // {Jack: 21}
    users.insert("Jill", 18); // {Jack: 21, Jill: 18}
    users.insert("Adam", 15); // {Jack: 21, Jill: 18, Adam: 15}

    for (person, account) in users.iter() {
        // .iter() will be explained in iterator chapter
        println!("person = {}, account = {:?}", person, account);
    }
}
