use std::fs::File;

fn main() {
    // File::open returns a Result<T, E>
    let f = File::open("/path/to/directory/of/hello.txt");

    let f = match f {
        Ok(file) => println!("File is present"),
        Err(error) => println!("Problem opening the file: {:?}", error), // best not to panic! best ask user to retry
    };
}
