use std::fs;

fn main() {

    let content = fs::read_to_string("./1.in")
        .expect("Make sure file exists");

    let mut x = 0;
    let mut flag = 0;

    for (i, c) in content.chars().enumerate() {
        match c {
            '(' => x = x + 1,
            ')' => x = x - 1,
             _  => (),
        }

        if x == -1 && flag == 0 {
            println!("entering basement at position {}", i + 1);
            flag = 1;
        }
    }
    println!("{x}");
}

    //fn main() {
    //
    //  let content = fs::read_to_string("./1.in")
//    .expect("Make sure file exists");
//
//  let mut x = 0;
//  let mut flag = 0;
//
//  for (i, c) in content.chars().enumerate() {
//    if c == '(' {
//        x = x + 1;
//      }
//    else if c == ')' {
//        x = x - 1;
//      }
//    if x == -1 && flag == 0 {
//        println!("entering basement at position {}", i + 1);
//        flag = 1;
//    }
//  }
//  println!("{x}");
//}
