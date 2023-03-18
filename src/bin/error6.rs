use std::fs::File;
use std::io::{self, Read};

fn try_open_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  match f {
      Ok(file) => return Ok("file can be opened".to_owned()),
      Err(e) => return Err(e), // return propagates the error to the calling function
  };
}

fn main() {
let r = try_open_file();
       match r {
         Ok(file) => // do the necessary
         Err(e) => // do the necessary
       };
}
