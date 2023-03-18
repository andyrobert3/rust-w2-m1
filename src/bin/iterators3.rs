fn main() {
    // Using For...in
    let numbers = vec![1, 2, 3, 4, 5];
    let mut plus_one = vec![];
    for num in numbers {
        plus_one.push(num + 1);
    }

    // prints [2, 3, 4, 5, 6]
    println!("{:?}", plus_one);
    
    // Using iterator
    let numbers = vec![1, 2, 3, 4, 5];
    let plus_one: Vec<_> = numbers
        .iter() // converts vector to iterator
        .map(|num| num + 1) // increment each item in iterator
        .collect(); // converts iterator to vector

    // prints [2, 3, 4, 5, 6]
    println!("{:?}", plus_one);
}
