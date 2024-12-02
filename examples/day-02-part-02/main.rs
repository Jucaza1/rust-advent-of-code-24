use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let file = File::open("./examples/day-02-part-02/input-test2.txt").expect("file not found");
    // let file = File::open("./examples/day-02-part-02/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-02-part-02/input.txt").expect("file not found");
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
    match has_unsafe(&levels_vec) {
        Some(i) => {
            if has_unsafe(&levels_vec.omit_index(i)).is_none()
                || i > 0 && has_unsafe(&levels_vec.omit_index(i - 1)).is_none()
                || i > 1 && has_unsafe(&levels_vec.omit_index(i - 2)).is_none()
            {
                return true;
            }
        }
        None => return true,
    }
    false
}
fn has_unsafe(v: &[i32]) -> Option<usize> {
    let mut level_last = v.first().expect("theres first number");
    let mut diff = Diff::Zero;
    println!("{v:?}");
    for (i, level) in v.iter().enumerate().skip(1) {
        println!("index {i}");
        match (&diff, level - level_last) {
            (Diff::Pos, 1..=3) => (),
            (Diff::Neg, -3..=-1) => (),
            (Diff::Zero, 1..=3) => diff = Diff::Pos,
            (Diff::Zero, -3..=-1) => diff = Diff::Neg,
            (_, _) => return Some(i),
        }
        level_last = level;
    }
    None
}
enum Diff {
    Zero,
    Pos,
    Neg,
}
trait OmitIndex<T> {
    fn omit_index(&self, i: usize) -> Vec<T>;
}
impl<T> OmitIndex<T> for Vec<T>
where
    T: Copy,
{
    fn omit_index(&self, i: usize) -> Vec<T> {
        if i >= self.len() {
            self.to_vec()
        } else {
            self.iter()
                .enumerate()
                .filter(|(indx, _)| i != *indx)
                .map(|(_, x)| *x)
                .collect::<Vec<T>>()
        }
    }
}
