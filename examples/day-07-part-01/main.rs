use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let file = File::open("./examples/day-07-part-01/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-07-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut equations: Vec<Vec<i64>> = Vec::new();
    for line in buff.lines() {
        let Ok(mut line) = line else {
            unreachable!();
        };
        line = line.replace(": ", " ");
        equations.push(
            line.split(" ")
                .map(|x| {
                    println!("{x}");
                    x.parse::<i64>().expect("error parsing number")
                })
                .collect(),
        )
    }
    let mut acc = 0i64;
    'outer: for equation in equations.iter() {
        println!("eq ->{equation:?}");
        let mut operations: Vec<Operator> =
            equation.iter().skip(2).map(|_| Operator::Sum).collect();
        for n in 0..2i64.pow(operations.len() as u32) {
            let mut rem = n;
            for op in operations.iter_mut() {
                if rem%2 == 0 {
                    *op=Operator::Sum;
                } else {
                    *op=Operator::Mul;
                }
                rem /= 2;
            }
            let mut res_acc = equation[1];
            for i in 2..equation.len() {
                res_acc = match operations[i - 2] {
                    Operator::Sum => res_acc + equation[i],
                    Operator::Mul => res_acc * equation[i],
                }
            }
            println!("{operations:?}");
            if res_acc == equation[0] {
                println!("PASS ->{equation:?}");
                acc += equation[0];
                continue 'outer;
            }
        }
    }

    println!("Result -> {}", acc);
}
#[derive(Debug)]
enum Operator {
    Sum,
    Mul,
}
