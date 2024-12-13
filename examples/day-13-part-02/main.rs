use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

const BUTTON_A: &str = "Button A";
const BUTTON_B: &str = "Button B";
const PRIZE: &str = "Prize";
const PREFIX: [char; 5] = [' ', 'X', 'Y', '+', '='];
const TOKENS: (i64, i64) = (3, 1);

fn main() {
    // let file = File::open("./examples/day-13-part-02/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-13-part-02/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut claw_machines: Vec<ClawMachine> = Vec::new();
    let mut claw_parser: ClawMachine = ClawMachine {
        a: (0, 0),
        b: (0, 0),
        p: (0, 0),
    };

    for line in buff.lines() {
        let Ok(line) = line else {
            unreachable!();
        };
        if line.is_empty() {
            continue;
        }
        let mut line_split = line.split(":");
        let (Some(tag), Some(data), None) =
            (line_split.next(), line_split.next(), line_split.next())
        else {
            unreachable!();
        };
        let mut data_split = data.split(",");
        let (Some(x), Some(y), None) = (
            data_split
                .next()
                .and_then(|x| x.trim_start_matches(PREFIX).parse::<i64>().ok()),
            data_split
                .next()
                .and_then(|x| x.trim_start_matches(PREFIX).parse::<i64>().ok()),
            data_split.next(),
        ) else {
            panic!("error parsing data");
        };
        match tag {
            BUTTON_A => {
                claw_parser.a = (x, y);
            }
            BUTTON_B => {
                claw_parser.b = (x, y);
            }
            PRIZE => {
                claw_parser.p = (x + 10000000000000, y + 10000000000000);
                claw_machines.push(claw_parser.clone());
            }
            _ => panic!("error found in match"),
        }
    }
    // println!("{:#?}",claw_machines);
    let mut acc = 0i64;
    for claw_machine in claw_machines.iter() {
        if let Some(res) = find_minimun_tokens(claw_machine) {
            // println!("On machine {:#?}", claw_machine);
            // println!("won a prize for {}", res);
            acc += res;
        }
    }

    println!("Result -> {}", acc);
}
fn find_minimun_tokens(claw_machine: &ClawMachine) -> Option<i64> {
    let mut min_opt: Option<(i64, i64)> = None;
    find_minimum_cost(claw_machine, &mut min_opt);
    min_opt.map(|(a, b)| a * TOKENS.0 + b * TOKENS.1)
}
fn find_minimum_cost(claw_machine: &ClawMachine, min_opt: &mut Option<(i64, i64)>) {
    // println!("current = {:?}",current);
    // let _ = io::stdout().flush();
    // b = (yp  * xa / ya - xp / (yb * xa / ya - xb))
    // a = (xp  - b * xb) / xa
    let b = (claw_machine.p.1 as f64 * claw_machine.a.0 as f64 / claw_machine.a.1 as f64
        - claw_machine.p.0 as f64)
        / (claw_machine.b.1 as f64 * (claw_machine.a.0 as f64 / claw_machine.a.1 as f64)
            - claw_machine.b.0 as f64);
    let a = (claw_machine.p.0 as f64 - b * claw_machine.b.0 as f64) / claw_machine.a.0 as f64;
    print!("a => {a}");
    println!(" b => {b}");
    let epsilon = 1e-2;
    if is_almost_integer(b, epsilon) && is_almost_integer(a, epsilon) && a >= 0.0 && b >= 0.0 {
        *min_opt = Some((a.round() as i64, b.round() as i64));
    }
}
fn is_almost_integer(n: f64, epsilon: f64) -> bool {
    // let proportional_epsilon = epsilon * n.abs();
    // (n - n.round()).abs() < proportional_epsilon
    (n - n.round()).abs() < epsilon
}
#[test]
fn almost_integer() {
    let epsilon = 1e-2;
    assert!(is_almost_integer(200488997589.00103, epsilon));
}
#[derive(Clone, Debug)]
struct ClawMachine {
    a: (i64, i64),
    b: (i64, i64),
    p: (i64, i64),
}
