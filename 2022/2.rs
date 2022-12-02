use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

//fn main() {
//
//    //Use a hashmap to hold relations
//    
//    let results = HashMap::from([
//        ("X", [1 + 3, 1 + 0, 1 + 6]),
//        ("Y", [2 + 6, 2 + 3, 2 + 0]),
//        ("Z", [3 + 0, 3 + 6, 3 + 3])
//    ]);
//
//
//    let file = File::open("2.in")
//        .expect("Make sure file exists");
//
//    let buffer = BufReader::new(file);
//    
//    let mut sum = 0;
//
//    for line in buffer.lines() {
//
//        let l = line.unwrap();
//
//        let vec: Vec<&str> = l.split(" ").collect();
//
//        match results.get(vec[1]) {
//            Some(array) => sum += array[(vec[0].chars().nth(0).unwrap() as u32 - 'A' as u32) as usize],
//            None        => println!("Missing key in hash"),
//        }
//
//    }
//    println!("{sum}", );
//}

fn main() {

    //Use a list to hold winning order of rock, paper, and scissors. 
    //n always beats n + 1

    let graph = [1, 3, 2]; // Rock -> Scissors -> Paper -> Rock
    
    //Holds the letter to index relationship. +3 since offsets can be negative.
    let shapes= HashMap::from([
        ("A", 0 + 3),
        ("B", 2 + 3),
        ("C", 1 + 3)
    ]);

    let file = File::open("2.in")
        .expect("Make sure file exists");

    let buffer = BufReader::new(file);
    
    let mut sum = 0;

    for line in buffer.lines() {

        let l = line.unwrap();

        let vec: Vec<&str> = l.split(" ").collect();

        match vec[1] {
            "X" => sum += graph[(shapes.get(vec[0]).unwrap() + 1) % 3] + 0,
            "Y" => sum += graph[(shapes.get(vec[0]).unwrap() + 0) % 3] + 3,
            "Z" => sum += graph[(shapes.get(vec[0]).unwrap() - 1) % 3] + 6,
             _  => println!("Missing key in hash"),
        }
    }
    println!("{sum}", );
}
