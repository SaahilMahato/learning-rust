pub fn run() {
    // Print to console
    println!("Hello from print.rs file");

    // print by using placeholders
    println!("{} is so {}", "Saahil", "lazy");

    // can print non-string data only by placeholder
    println!("Number: {} Another: {}", 1, 2);

    // print by placeholder and indexing
    println!("{0} is from {1} and {0} likes to {2}", "Saahil", "Bhaktapur", "code");

    // print by placeholder and named parameters
    println!("{name} like to play {activity}.", name="Saahil", activity="Video games");


    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math while printing
    println!("10 + 10 = {}", 10+10);
}

