// Used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("{}", count);

        if count == 20 {break;};
    }

    // While loop
    while count <= 100 {
        if count % 15 == 0 {
            println!("Fizz Buzz");
        }
        else if count % 3 == 0 {
            println!("Fizz");
        }
        else if count % 5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}", count);
        }
        count += 1;
    }

    // For loop
    for x in 0..10 {
        println!("{}", x);
    }
}