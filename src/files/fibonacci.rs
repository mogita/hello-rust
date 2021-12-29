fn main() {
    // prints fibonacci
    print_fibonacci_from(20, 30);
}

fn find_fibonacci_by(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => find_fibonacci_by(n - 1) + find_fibonacci_by(n - 2),
    }
}

fn print_fibonacci_from(start: u32, end: u32) {
    for p in start..end + 1 {
        println!("{}", find_fibonacci_by(p))
    }
}
