// Variables hold primitive data or references to data
// Variable are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Saahil";
    let mut age = 23; // make variable mutable
    println!("First age = {}", age);

    age = 24;

    println!("Name = {}, Age = {}", name, age);

    // Define constants
    const ID: i32 = 001; // constants need to have their type defined
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Saahil", 23);
    println!("Name: {}, Age: {}", my_name, my_age);
}