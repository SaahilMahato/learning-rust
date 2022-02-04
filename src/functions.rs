pub fn run() {
    greetings("Hello", "Saahil");
    greetings("Wattup", "Homie");

    // Bind function return values to variables
    let sum = add(5, 5);
    println!("{}", sum);

    // Closures
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(3, 3));
}

fn greetings(greet: &str, name: &str) {
    println!("{}, {}!!", greet, name);
}

fn add(num1: i32, num2: i32) -> i32 {  // -> return type
    num1 + num2 // no semicolon means return statement
}