fn main() {
    let vec = [-1, 0, 1];

    // create an iterator variable, must be mutable, as calling .next() changes its internal state
    let mut iterator = vec.iter();
    println!("{:?}", iterator.next()); // [-1, 0, 1] prints Some(-1)
    println!("{:?}", iterator.next()); // [0, 1] prints Some(0)
    println!("{:?}", iterator.next()); // [1] prints Some(1)
    println!("{:?}", iterator.next()); // [] prints None
    println!("");
}
