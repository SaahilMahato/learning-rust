// Use to compare values and get true or false

pub fn run() {
    let age: u8 = 22;
    let check_id: bool = true;

    if age >= 21 && check_id {
        println!("What would you like to drink?");
    }
    else if age < 21 && check_id {
        println!("Go home kid.");
    }
    else {
        println!("Show me your ID.");
    }

    let is_of_age: bool = if age>21 {true} else {false};
    println!("{}", is_of_age);
}