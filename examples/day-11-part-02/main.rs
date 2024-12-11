use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

fn main() {
    // let file = File::open("./examples/day-11-part-02/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-11-part-02/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut stone_map: HashMap<i64, i64> = HashMap::new();
    let Some(Ok(line)) = buff.lines().next() else {
        unreachable!();
    };
    line.split(" ").for_each(|x| {
        let n = x.parse::<i64>().expect("error parsing number");
        stone_map.entry(n).and_modify(|x| *x += 1).or_insert(1);
    });
    for _ in 0..75 {
        // print!("\rProgress = {}%", i * 100 / 75);
        // let _ = io::stdout().flush();
        let mut new_stone_map: HashMap<i64, i64> = HashMap::new();
        for (stone, count) in stone_map.iter() {
            if *stone == 0 {
                new_stone_map
                    .entry(1)
                    .and_modify(|x| *x += count)
                    .or_insert(*count);
                continue;
            }
            let n_vec = separate_digits(*stone);
            if n_vec.len() % 2 == 0 {
                new_stone_map
                    .entry(fuse_digits(&n_vec[..n_vec.len() / 2]))
                    .and_modify(|x| *x += count)
                    .or_insert(*count);
                new_stone_map
                    .entry(fuse_digits(&n_vec[n_vec.len() / 2..]))
                    .and_modify(|x| *x += count)
                    .or_insert(*count);
                continue;
            }
            new_stone_map
                .entry(stone * 2024)
                .and_modify(|x| *x += count)
                .or_insert(*count);
        }
        stone_map = new_stone_map;
    }
    // println!("{stone_map:?}");
    let mut acc = 0i64;
    for (_,count) in stone_map.iter(){
        acc += count;
    }

    println!("Result -> {}", acc);
}
fn separate_digits(mut n: i64) -> Vec<i64> {
    if n == 0 {
        return vec![0];
    }
    let mut n_vec: Vec<i64> = Vec::new();
    while n > 0 {
        n_vec.push(n % 10);
        n /= 10;
    }
    n_vec.reverse();
    n_vec
}
fn fuse_digits(v: &[i64]) -> i64 {
    let mut n = 0i64;
    for (i, x) in v.iter().rev().enumerate() {
        n += x * 10i64.pow(i as u32);
    }
    n
}
