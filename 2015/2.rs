use std::fs;
use std::cmp;
use itertools::Itertools;

fn main() {

  let content = fs::read_to_string("./1.in")
    .expect("Make sure file exists");

  let mut t = 0;
  let mut flag = 0;

  for line in content.lines() {
    let sides: Vec<&str> = line.split("x").collect();

    for side in sides {
      let mut min = 0;


      min = cmp::min(min, side);

      if c == '(' {
          x = x + 1;
        }
      else if c == ')' {
          x = x - 1;
        }
      if x == -1 && flag == 0 {
          println!("entering basement at position {}", i + 1);
          flag = 1;
      }

    }
  }
  println!("{x}");
}
