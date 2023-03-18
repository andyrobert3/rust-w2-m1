use std::fs::File;
use std::io::{self, Read};

fn try_open_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt")?;

    // let f = match f {
    //     Ok(file) => Ok("file can be opened".to_owned()),
    //     Err(e) => return Err(e), // return propagates the error to the calling function
    // };

    Ok("file can be opened".to_owned())
}

fn main() {
    try_open_file();
}
