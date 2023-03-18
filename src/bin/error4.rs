use std::fs::File;

fn main() {
    // if Ok variant is returned, unwrap() returns the value inside the Ok variant
    // if Err variant is returned, unwrap() calls the panic! macro
    let f = File::open("/path/to/directory/of/hello.txt").unwrap();

    // expect() is similar to unwrap() but allows a custom error message
    // this makes it easier to find the code that actually caused this error
    let f = File::open("/path/to/directory/of/hello.txt").expect("Could not find hello.txt!!!");
}
