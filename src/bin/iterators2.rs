fn main() {
    let vec = vec![-1, 0, 1];

    // A typical for loop
    println!("A typical for loop");
    for x in &vec {
        println!("{}", x); // This prints -1, 0, 1
    }

    // Using a for loop with iter(), same output as above
    println!("Using a for loop with iter()");
    for x in vec.iter() {
        println!("{}", x); // This prints -1, 0, 1
    }

    // Using a filter() with Closure syntax
    println!("Using a filter() with Closure syntax");
    for x in vec.iter().filter(|x| **x > 0) {
        // Filter values that are more than 0
        println!("{}", x); // This prints 1
    }
}
