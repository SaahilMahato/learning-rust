// structs - Used to create a custom data types

// Normal structure
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple structure
struct RGB(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // get full name
    fn get_full_name(&self) -> String {
        format!("{} {}.", self.first_name, self.last_name)
    }

    // get last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    let rgb: RGB = RGB(255, 0, 255);

    let mut p = Person::new("John", "Doe");

    println!("Color: {} {} {}", c.red, c.green, c.blue);
    println!("RGB: {} {} {}", rgb.0, rgb.1, rgb.2);
    println!("Person {} {}", p.first_name, p.last_name);
    println!("The name's {}", p.get_full_name());
    p.set_last_name("Mahato");
    println!("The name's {}", p.get_full_name());
    println!("Person {:?}", p.to_tuple());
}