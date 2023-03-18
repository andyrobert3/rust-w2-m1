use std::fs::File;

fn main() {
    // File::open returns a Result<T, E>
    let f = File::open("./hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error), // best not to panic! best ask user to retry
    };
}
