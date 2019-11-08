use std::io::stdin;

fn main() {
  println!("Convert cel <=> fah");
  loop {
    println!("Input f{{number}} as fah => cel, or c{{number}} as cel => fah:");
    let mut input = String::new();
    stdin()
      .read_line(&mut input)
      .expect("[Error] Failed to readline");
    let original_type = input.trim().chars().next().expect("original_type error");
    let value = remove_first_char(&input.trim()).expect("value error");
    let value: i32 = match value.parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    if original_type == 'c' {
      let conv_value = value * 9 / 5 + 32;
      println!("{}º cel is {}º fah", value, conv_value);
    } else if original_type == 'f' {
      let conv_value = (value - 32) * 5 / 9;
      println!("{}º fah is {}º cel", value, conv_value);
    } else {
      println!("Only accepts c or f for the first char");
    }
  }
}

fn remove_first_char(s: &str) -> Option<&str> {
  s.chars().next().map(|c| &s[c.len_utf8()..])
}
