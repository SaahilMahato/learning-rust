// Primitive string: Ungrowable fixed-length string
// String: Growable, heap-allocated data structure


pub fn run() {
    let hello = "hello";
    let mut growable_hello = String::from("Hello ");

    println!("{} {}", hello, growable_hello);

    println!("{}", growable_hello.len());

    // Push char
    growable_hello.push('W');
    println!("{}", growable_hello);

    // Push string
    growable_hello.push_str("orld!");
    println!("{}", growable_hello);

    //Capacity in bytes
    println!("Capacity: {}", growable_hello.capacity());

    // Check if empty
    println!("Is empty: {}", growable_hello.is_empty());

    // Contains substring
    println!("{}", growable_hello.contains("orld"));

    // Replace
    println!("Replace: {}", growable_hello.replace("World", "Mars"));

    // Loop through words in a string
    for word in growable_hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut str_with_capacity = String::with_capacity(10);
    str_with_capacity.push('A');
    str_with_capacity.push('B');
    println!("{}", str_with_capacity);

    // Assertion testing
    assert_eq!(10, str_with_capacity.capacity());
}