// create get_first_element<T> function taking in (vec: Vec<T>) and outputs a Option<T> type
// combine and use into_iter() and next() to obtain first element
use std::vec;

// How to convert this into "&Vec<T>" reference?
fn get_first_element<T: Clone>(vec: &Vec<T>) -> Option<T> {
    vec.into_iter().next().map(|val| val.clone())
}

fn main() {
    // create vec1 with 3 number items
    // create vec2 with 0 item
    let vec1: Vec<f64> = vec![1.0, 2.0, 3.0];
    let vec2: Vec<i32> = vec![];

    // obtain first_element1 by passing in vec1 into get_first_element
    // obtain first_element2 by passing in vec2 into get_first_element

    let first_element1 = get_first_element(&vec1);
    let first_element2 = get_first_element(&vec2);

    // match on first_element1
    // if Some, print "The first element is <first item>"
    // if None, print "The vector is empty!"

    match first_element1 {
        Some(elem) => println!("The first element is {}", elem),
        None => println!("The vector is empty")
    }

    // match on first_element2
    // if Some, print "The first element is <first item>"
    // if None, print "The vector is empty!"
    match first_element2 {
        Some(elem) => println!("The first element is {}", elem),
        None => println!("The vector is empty")
    }
}
