use core::panic;
use std::{
    fs::File,
    io::{BufReader, Read},
};
const START: [char; 4] = ['m', 'u', 'l', '('];
const SEP: [char; 1] = [','];
const END: [char; 1] = [')'];
const DISABLE: [char; 7] = ['d', 'o', 'n', '\'', 't', '(', ')'];
const ENABLE: [char; 4] = ['d', 'o', '(', ')'];

fn main() {
    // let file = File::open("./examples/day-03-part-02/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-03-part-02/input.txt").expect("file not found");
    let mut buff = BufReader::new(file);
    let mut line: String = String::new();
    let Ok(_) = buff.read_to_string(&mut line) else{
        panic!("error reading the file");
    };
    let acc = process_line(&line);
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
                } else if c == DISABLE[0] {
                    j = 1;
                    state = State::Disabling;
                    continue;
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
            State::Disabling => {
                if c == DISABLE[j] {
                    j += 1;
                } else if c == START[0] {
                    j = 1;
                    state = State::Start;
                    continue;
                } else {
                    j = 0;
                    state = State::Start;
                    continue;
                }
                if j == START.len() {
                    j = 0;
                    state = State::Enabling;
                }
            }
            State::Enabling => {
                if c == ENABLE[j] {
                    j += 1;
                } else if c == ENABLE[0] {
                    j = 1;
                    continue;
                } else {
                    j = 0;
                    continue;
                }
                if j == ENABLE.len() {
                    j = 0;
                    state = State::Start
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
                    } else if c == DISABLE[0] {
                        state = State::Disabling;
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
                    if c == START[0] {
                        state = State::Start;
                        j = 1;
                    } else if c == DISABLE[0] {
                        state = State::Disabling;
                        j = 1;
                    } else {
                        state = State::Start;
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
    Disabling,
    Enabling,
    N1,
    Sep,
    N2,
    End,
}
