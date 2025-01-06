use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let file = File::open("./examples/day-17-part-01/input-test.txt").expect("file not found");
    // let file = File::open("./examples/day-17-part-01/input-test2.txt").expect("file not found");
    let file = File::open("./examples/day-17-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut state = StateParse::Reg(0);
    let mut prog = Vec::<i64>::new();
    let mut machine = Machine { a: 0, b: 0, c: 0 };
    for line in buff.lines() {
        let Ok(line) = line else {
            panic!("error reading line");
        };
        if line.is_empty() {
            state = StateParse::Prog;
            continue;
        }
        match &state {
            StateParse::Reg(0) => {
                machine.a = line
                    .split_once(": ")
                    .map(|x| x.1)
                    .and_then(|x| x.parse::<i64>().ok())
                    .expect("error parsing reg");
                state = StateParse::Reg(1);
            }
            StateParse::Reg(1) => {
                machine.b = line
                    .split_once(": ")
                    .map(|x| x.1)
                    .and_then(|x| x.parse::<i64>().ok())
                    .expect("error parsing reg");
                state = StateParse::Reg(2);
            }
            StateParse::Reg(_) => {
                machine.c = line
                    .split_once(": ")
                    .map(|x| x.1)
                    .and_then(|x| x.parse::<i64>().ok())
                    .expect("error parsing reg");
                state = StateParse::Prog;
            }
            StateParse::Prog => {
                prog = line
                    .split_once(": ")
                    .map(|x| x.1)
                    .and_then(|x| x.split(",").map(|y| y.parse::<i64>().ok()).collect())
                    .expect("error parsing prog");
            }
        }
    }
    println!("machine {machine:?}");
    println!("prog {prog:?}");
    print!("Result -> ");
    machine.process(&prog).iter().for_each(|x| print!("{x},"));
}
enum StateParse {
    Reg(u8),
    Prog,
}
#[derive(Debug)]
struct Machine {
    a: i64,
    b: i64,
    c: i64,
}
impl Machine {
    fn process(&mut self, input: &[i64]) -> Vec<i64> {
        let mut output = Vec::<i64>::new();
        let mut i = 0usize;
        while i < input.len() - 1 {
            let operand = match input[i + 1] {
                4 => self.a,
                5 => self.b,
                6 => self.c,
                x => x,
            };
            match input[i] {
                0 => self.a /= 2i64.pow(operand as u32),
                1 => self.b ^= input[i+1],
                2 => self.b = operand % 8,
                3 => {
                    if self.a != 0 {
                        i = input[i+1] as usize;
                        continue;
                    }
                }
                4 => self.b ^= self.c,
                5 => output.push(operand % 8),
                6 => self.b = self.a / 2i64.pow(operand as u32),
                _ => self.c = self.a / 2i64.pow(operand as u32),
            }
            i += 2;
        }
        output
    }
}
