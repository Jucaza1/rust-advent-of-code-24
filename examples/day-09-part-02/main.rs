use core::panic;
use std::io::{self, Write};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let file = File::open("./examples/day-09-part-02/input-test.txt").expect("file not found");
    // let file = File::open("./examples/day-09-part-02/input-test2.txt").expect("file not found");
    let file = File::open("./examples/day-09-part-02/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut buff_lines = buff.lines();
    let (Some(Ok(line)), None) = (buff_lines.next(), buff_lines.next()) else {
        unreachable!();
    };
    let mut storage: Vec<Option<i64>> = Vec::new();
    let mut id_map: HashMap<i64, (usize, usize)> = HashMap::new();
    let mut free_index_map: HashMap<usize, usize> = HashMap::new();
    let mut file_id_max: i64 = 0;
    {
        let mut state = State::File;
        let mut i = 0i64;
        for c in line.chars() {
            let Some(n) = c.to_digit(10) else {
                panic!("char is not a digit");
            };
            match &state {
                State::File => {
                    for _ in 0..n {
                        storage.push(Some(i));
                    }
                    id_map.insert(i, (storage.len() - n as usize, n as usize));
                    state = State::Free;
                    i += 1;
                    file_id_max = i;
                }
                State::Free => {
                    for _ in 0..n {
                        storage.push(None);
                    }
                    free_index_map.insert(storage.len() - n as usize, n as usize);
                    state = State::File;
                }
            }
        }
    }
    // print_vec(&storage);
    for file_id in (1..file_id_max).rev() {
        let Some((file_index, file_len)) = id_map.get(&file_id) else {
            continue;
        };
        print!("\rProgress: {}%", (file_id_max-file_id)*100/file_id_max);
        io::stdout().flush().unwrap();
        for free_index in 0..storage.len() {
            let Some(free_len) = free_index_map.get(&free_index) else {
                continue;
            };
            if file_index < &free_index{
                break;
            }
            if file_len <= free_len {
                for i in 0..*file_len {
                    storage[free_index + i] = Some(file_id);
                    storage[file_index + i] = None;
                }
                if file_len < free_len {
                    free_index_map.insert(free_index + file_len , free_len - file_len);
                }
                free_index_map.remove(&free_index);
                // print_vec(&storage);
                break;
            }
        }
    }
    println!();
    let acc = storage
        .iter()
        .enumerate()
        .map(|(i, x)| match &x {
            Some(n) => n * i as i64,
            None => 0,
        })
        .sum::<i64>();
    println!("Result -> {}", acc);
}
fn print_vec(m: &[Option<i64>]) {
    for row in m.iter() {
        match row {
            Some(x) => print!("{x}"),
            None => print!("."),
        }
    }
    println!();
}
enum State {
    File,
    Free,
}
