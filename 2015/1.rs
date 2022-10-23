use std::fs;

fn main() {

  let content = fs::read_to_string("./1.in")
    .expect("Make sure file exists");

  let mut x = 0;

  for c in content.chars() {
    if c == '(' {
        x = x + 1;
      }
    else if c == ')' {
        x = x - 1;
      }
  }

  println!("{x}\n");

}
