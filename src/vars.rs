pub fn run() {
    let name = "Chris";
    let mut age = 34;
    println!("My name is {} and I am {}", name, age);
    age = 35;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Chris", 34);
    println!("Name {}, age {}", my_name, my_age);
}
