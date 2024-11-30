use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./examples/day-01-part-02/input-test.txt").expect("file not found");
    // let file = File::open("./examples/day-01-part-02/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut acc = 0;
    for line in buff.lines() {
        let Ok(line) = line else{
            panic!("line not present")
        };
        acc += process_line(&line)
    }
    println!("Result -> {}",acc);
}
fn process_line(s: &str) -> i32 {
    0
}
