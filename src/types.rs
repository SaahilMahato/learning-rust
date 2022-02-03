/*
Primitive data types in rust:
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
    based on number of bits they reserve in memory
    u: unsinged integer (can't store negative value)

Floats: f32, f64

Boolean: (bool) true or false

Characters (char)

Tuples

Arrays
*/

/*
Rust is statically typed language, which means that it must know
the types of all variables at compile time. However, the compiler can
usually infer the data type based on the values assigned
so we don't need to explicitly mention the data type while declaring variables.
*/

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 342534424234;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max i128: {}", std::i128::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 11;

    // Character
    let char_1: char = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, char_1, face));
}