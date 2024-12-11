use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let file = File::open("./examples/day-11-part-01/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-11-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut stones: Vec<i64>;
    let Some(Ok(line)) = buff.lines().next() else {
        unreachable!();
    };
    stones = line
        .split(" ")
        .map(|x| x.parse::<i64>().expect("error parsing number"))
        .collect();
    for _ in 0..25 {
        let mut new_stones: Vec<i64> = Vec::new();
        for stone in stones.iter() {
            if *stone == 0 {
                new_stones.push(1);
                continue;
            }
            let n_vec = separate_digits(*stone);
            if n_vec.len()%2 ==0{
                new_stones.push(fuse_digits(&n_vec[..n_vec.len()/2]));
                new_stones.push(fuse_digits(&n_vec[n_vec.len()/2..]));
                continue;
            }
            new_stones.push(stone*2024);
        }
        stones = new_stones;
    }

    println!("Result -> {}", stones.len());
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
    for (i,x) in v.iter().rev().enumerate(){
        n += x*10i64.pow(i as u32);
    }
    n
}
