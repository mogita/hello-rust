pub fn run() {
    greeting("Hello", "Chris");

    // Bind functino values to variables
    let get_sum = add(32, 4);
    println!("Sum: {}", get_sum);

    // Closure
    let p: i32 = 10;
    let add_nums = |m: i32, n: i32| m + n + p;
    println!("Closure sum: {}", add_nums(3, 5));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(m: i32, n: i32) -> i32 {
    // Without a semicolon, it's a "return"
    m + n
}
