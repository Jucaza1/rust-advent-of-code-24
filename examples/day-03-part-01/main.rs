use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
const START: [char; 4] = ['m', 'u', 'l', '('];
const SEP: [char; 1] = [','];
const END: [char; 1] = [')'];

fn main() {
    // let file = File::open("./examples/day-03-part-01/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-03-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut acc = 0;
    for line in buff.lines() {
        let Ok(line) = line else {
            panic!("line not present")
        };
        acc += process_line(&line)
    }
    println!("Result -> {}", acc);
}
fn process_line(s: &str) -> i32 {
    let mut acc = 0i32;
    let mut state = State::Start;
    let s_chars = s.chars();
    let mut j = 0usize;
    let mut n1 = 0i32;
    let mut n2 = 0i32;
    for c in s_chars {
        match &state {
            State::Start => {
                if c == START[j] {
                    j += 1;
                } else {
                    if c == START[0] {
                        j = 1;
                    } else {
                        j = 0;
                    }
                    continue;
                }
                if j == START.len() {
                    j = 0;
                    state = State::N1
                }
            }
            State::N1 => {
                if c == SEP[0] {
                    if SEP.len() == 1 {
                        state = State::N2;
                    } else {
                        state = State::Sep;
                    }
                    continue;
                }
                let Some(n) = c.to_digit(10).map(|x| x as i32) else {
                    state = State::Start;
                    n1 = 0;
                    if c == START[0] {
                        j = 1;
                    } else {
                        j = 0;
                    }
                    continue;
                };
                n1 = n1 * 10 + n;
            }
            State::Sep => {
                if c == SEP[j + 1] {
                    j += 1;
                } else {
                    state = State::Start;
                    if c == START[0] {
                        j = 1;
                    } else {
                        j = 0;
                    }
                    (n1, n2) = (0, 0);
                    continue;
                }
                if j == SEP.len() - 1 {
                    j = 0;
                    state = State::N2;
                }
            }
            State::N2 => {
                if c == END[0] {
                    j = 0;
                    if END.len() == 1 {
                        state = State::Start;
                        acc += n1 * n2;
                        (n1, n2) = (0, 0);
                    } else {
                        state = State::End;
                    }
                    continue;
                }
                let Some(n) = c.to_digit(10).map(|x| x as i32) else {
                    state = State::Start;
                    (n1, n2) = (0, 0);
                    if c == START[0] {
                        j = 1;
                    }
                    continue;
                };
                n2 = n2 * 10 + n;
            }
            State::End => {
                if c == END[j + 1] {
                    j += 1;
                } else {
                    state = State::Start;
                    if c == START[0] {
                        j = 1;
                    } else{
                        j = 0;
                    }
                    continue;
                }
                if j == END.len() - 1 {
                    j = 0;
                    state = State::Start;
                    acc += n1 * n2;
                    (n1, n2) = (0, 0);
                }
            }
        }
    }
    acc
}
enum State {
    Start,
    N1,
    Sep,
    N2,
    End,
}
