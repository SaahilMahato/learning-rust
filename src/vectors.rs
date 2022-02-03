// Vectors are resizable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3];

    // Re-assign value
    numbers[2] = 20;

    //Add on to vector
    numbers.push(4);
    numbers.push(5);

    //Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // get sinlge element
    println!("First element: {}", numbers[0]);

    // get vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers: {:?}", numbers);
}