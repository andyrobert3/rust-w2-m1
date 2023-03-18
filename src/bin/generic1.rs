struct MultipliersI32 {	// i32 type
   x: i32,
   y: i32,
}

struct MultipliersF32 {	// f32 type	
   x: f32,
   y: f32,
}

struct MultipliersU32 {	// u32 type	
   x: u32,
   y: u32,
}

 
 fn main() {
    let i32_multiplier : MultipliersI32 = MultipliersI32 { x: 5, y: -10 }; // valid, T has type of i32
    let f32_multiplier : MultipliersF32 = MultipliersF32 { x: 5.0, y: 10.0 }; // valid, T has type of i32
    let u32_multiplier : MultipliersU32 = MultipliersU32 { x: 5, y: 10 }; // valid, T has type of i32
 }