use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let file = File::open("./examples/day-02-part-01/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-02-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut acc = 0;
    for line in buff.lines() {
        let Ok(line) = line else {
            panic!("line not present")
        };
        if process_line(&line) {
            acc += 1;
        }
    }
    println!("Result -> {}", acc);
}
fn process_line(s: &str) -> bool {
    let levels_vec: Vec<i32> = s.split(" ").filter_map(|x| x.parse::<i32>().ok()).collect();
    let mut level_last = levels_vec.first().expect("theres first number");
    let mut diff = Diff::Zero;
    for level in levels_vec.iter().skip(1) {
        match (&diff, level - level_last) {
            (Diff::Pos, 1..=3) => (),
            (Diff::Neg, -3..=-1) => (),
            (Diff::Zero, 1..=3) => diff = Diff::Pos,
            (Diff::Zero, -3..=-1) => diff = Diff::Neg,
            (_, _) => return false,
        }
        level_last = level;
    }
    true
}
enum Diff {
    Zero,
    Pos,
    Neg,
}
