// Structs - Used to create custom data types

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct ColorT(u8, u8, u8);

// Struct with functions
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct the person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to Tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color{
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c = ColorT(255, 0, 0);
    c.0 = 244;
    println!("ColorT: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("John", "Doe");
    p.last_name = String::from("Wang");
    println!("Person: {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person: {}", p.full_name());

    println!("Person tuple name: {:?}", p.to_tuple());

    // Can't do this after to_tuple anymore
    // println!("Person: {}", p.full_name());
}
