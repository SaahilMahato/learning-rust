// Tuples are grouped together values that can be of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, u8) = ("Saahil", "Mahato", 23);

    println!("{} {} is {} years old", person.0, person.1, person.2);
}