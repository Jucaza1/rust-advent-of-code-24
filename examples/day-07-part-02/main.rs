use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let file = File::open("./examples/day-07-part-02/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-07-part-02/input.txt").expect("file not found");
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
                    x.parse::<i64>().expect("error parsing number")
                })
                .collect(),
        )
    }
    let mut acc = 0i64;
    'outer: for equation in equations.iter() {
        // println!("eq ->{equation:?}");
        let mut operations: Vec<Operator> =
            equation.iter().skip(2).map(|_| Operator::Sum).collect();
        for n in 0..3i64.pow(operations.len() as u32) {
            let mut rem = n;
            for op in operations.iter_mut() {
                if rem % 3 == 0 {
                    *op = Operator::Sum;
                } else if rem % 3 == 1 {
                    *op = Operator::Mul;
                } else {
                    *op = Operator::Con;
                }
                rem /= 3;
            }
            let mut res_acc = equation[1];
            for i in 2..equation.len() {
                res_acc = match operations[i - 2] {
                    Operator::Sum => res_acc + equation[i],
                    Operator::Mul => res_acc * equation[i],
                    Operator::Con => {
                        let mut exp = 1;
                        loop {
                            if 10i64.pow(exp) > equation[i] {
                                break;
                            }
                            exp += 1;
                        }
                        res_acc*10i64.pow(exp) + equation[i]
                    }
                }
            }
            // print!("{operations:?}");
            // println!("{res_acc}");
            if res_acc == equation[0] {
                // println!("PASS ->{equation:?}");
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
    Con,
}
