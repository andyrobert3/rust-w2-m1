fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for arrays yields `&i32`, find yields &&i32
    println!("Find 2 in vec1: {:?}", vec1.iter().find(|x| **x == 2));
    // prints: Find 2 in vec1: Some(2)

    // `into_iter()` for arrays yields `i32`, find yields &i32
    println!("Find 2 in vec2: {:?}", vec2.into_iter().find(|x| *x == 2));
    // prints: Find 2 in vec2: None
}
