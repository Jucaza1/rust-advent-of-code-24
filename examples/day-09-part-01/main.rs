use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./examples/day-09-part-01/input-test.txt").expect("file not found");
    // let file = File::open("./examples/day-09-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut buff_lines = buff.lines();
    let (Some(Ok(line)),None) = (buff_lines.next(), buff_lines.next()) else {
        unreachable!();
    };
    let mut storage: Vec<Option<i64>> = Vec::new();
    {
        let mut state = State::File;
        let mut i = 0i64;
        for c in line.chars(){
            let Some(n) = c.to_digit(10) else {
            panic!("char is not a digit");
        };
            match &state{
                State::File => {
                    for _ in 0..n{
                        storage.push(Some(i));
                    }
                    state = State::Free;
                    i +=1;
                }
                State::Free => {
                    for _ in 0..n{
                        storage.push(None);
                    }
                    state = State::File;
                }
            }
        }
    }
    let mut left = 0usize;
    let mut right = storage.len()-1;
    while left< right{
        match (&storage[left],&storage[right]){
            (None,Some(n)) => {
                storage[left] = Some(*n);
                storage[right] = None;
            }
            (Some(_),None) => {
                right -= 1;
                left += 1;
            }
            (None,None) => {
                right -= 1;
            }
            (Some(_),Some(_)) => {
                left += 1;
            }
        }
    }
    let acc = storage.iter().enumerate().map_while(|(i,x)| x.as_ref().map(|n|n*i as i64)).sum::<i64>();
    println!("Result -> {}", acc);
}
fn print_matix(m: &[Vec<Option<char>>]) {
    for row in m.iter() {
        for col in row.iter() {
            match col {
                Some(x) => print!("{x}"),
                None => print!("."),
            }
        }
        println!();
    }
}
enum State{
    File,
    Free,
}
