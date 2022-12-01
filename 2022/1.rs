use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::BinaryHeap;

fn main() {

    let file = File::open("1.in")
        .expect("Make sure file exists");

    let buffer = BufReader::new(file);
    
    let mut heap = BinaryHeap::new();

    let mut s = 0;

    for line in buffer.lines() {

        let l = line.unwrap();
        match l.as_str() {
            "" => { 
                    if heap.len() == 3 {
                        if heap.peek() > Some(&(s * (-1))) {
                            heap.pop();
                            heap.push(s * (-1));
                        }
                    }
                    else{
                            heap.push(s * (-1));
                    }
                    s = 0;
            },
            _  => s += l.parse::<i32>().unwrap(),
        }

    }
    println!("{}", heap.iter().min().unwrap() * (-1));
    println!("{}", heap.iter().sum::<i32>() * (-1));
}
