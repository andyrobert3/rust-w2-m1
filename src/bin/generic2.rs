struct Multipliers01<T> {			// Single generic types
    x: T,
    y: T,
 }
 
 struct Multipliers02<T, U> {		// Multiple generic types
    x: T,
    y: U,
 }
 
 fn main() {
    let integer:Multipliers01<i32> = Multipliers01 { x: 5, y: 10 }; // valid, T has type of i32
    let abc = Multipliers01 { x: 1.0, y: 4.0 }; // valid inferred type, T has type of f32
    let mixed = Multipliers01 {x: 1.0, y: 6}; // error: type mismatch, T cannot be both f32 and i32
    let mixed = Multipliers02 {x: 1.0, y: 6}; // valid with Multipliers02 struct, T = f32 and U = i32
 }