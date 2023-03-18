fn main() {
    let v = vec![1, 2, 3];
    v[99];      // this will cause a panic as item index 99 does not exist!
 }
 
 // this prints out a failure message:
 // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/bin/handlingerrors01.rs:7:5
 // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
 