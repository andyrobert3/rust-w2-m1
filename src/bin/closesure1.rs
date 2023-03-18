fn main() {
    let outer_variable = 6; // variable value from environment
    let add_outer_variable = |x: i32| x + outer_variable; // closure using outer_variable
    println!("{}", add_outer_variable(5)); // prints 11

    // Note ownership rules for complex Data Types is moved when using Closures, similar to Functions
    let outer_string = "Test".to_owned();
    let add_outer_string = |s: String| println!("{:?}", s);
    add_outer_string(outer_string); //String ownership is moved here
    println!("{:?}", outer_string); //This will error
}
