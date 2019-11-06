use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
  println!("guess the number!");
  let secret_number = rand::thread_rng().gen_range(1, 101);
  loop {
    println!("please input your guess:");
    let mut guess = String::new();
    stdin().read_line(&mut guess).expect("failed to read line");
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("too small!"),
      Ordering::Equal => {
        println!("you win!");
        break;
      }
      Ordering::Greater => println!("too big!"),
    }
  }
}
