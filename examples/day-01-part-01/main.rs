use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let file = File::open("./examples/day-01-part-01/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-01-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut acc = 0;
    let (mut left_v, mut right_v): (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new());
    for line in buff.lines() {
        let Ok(line) = line else {
            panic!("line not present")
        };
        let mut split_line = line.split("   ");
        let (Some(left_n), Some(right_n), None) = (
            split_line.next().and_then(|x| x.parse::<i32>().ok()),
            split_line.next().and_then(|x| x.parse::<i32>().ok()),
            split_line.next(),
        ) else {
            panic!("expected 2 numbers separated with 3 spaces")
        };
        left_v.push(left_n);
        right_v.push(right_n);
    }
    left_v.sort();
    right_v.sort();
    for i in 0..left_v.len() {
        acc += (left_v.get(i).expect("mismatch length") - right_v.get(i).expect("mismatch length"))
            .abs();
    }
    println!("Result -> {}", acc);
}
