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
const TOKENS: (i32, i32) = (3, 1);

fn main() {
    let file = File::open("./examples/day-13-part-01/input-test.txt").expect("file not found");
    // let file = File::open("./examples/day-13-part-01/input-test2.txt").expect("file not found");
    // let file = File::open("./examples/day-13-part-01/input.txt").expect("file not found");
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
                .and_then(|x| x.trim_start_matches(PREFIX).parse::<i32>().ok()),
            data_split
                .next()
                .and_then(|x| x.trim_start_matches(PREFIX).parse::<i32>().ok()),
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
                claw_parser.p = (x, y);
                claw_machines.push(claw_parser.clone());
            }
            _ => panic!("error found in match"),
        }
    }
    // println!("{:#?}",claw_machines);
    let mut acc = 0i32;
    for claw_machine in claw_machines.iter() {
        if let Some(res) = find_minimun_tokens(claw_machine) {
            // println!("On machine {:#?}", claw_machine);
            // println!("won a prize for {}", res);
            acc += res;
        }
    }

    println!("Result -> {}", acc);
}
fn find_minimun_tokens(claw_machine: &ClawMachine) -> Option<i32> {
    let mut min_opt: Option<(i32, i32)> = None;
    let mut seen: HashSet<(i32,i32)> = HashSet::new();
    recurse_find_minimum_cost((0, 0), claw_machine, &mut min_opt, &mut seen);
    min_opt.map(|(a, b)| a * TOKENS.0 + b * TOKENS.1)
}
fn recurse_find_minimum_cost(
    current: (i32, i32),
    claw_machine: &ClawMachine,
    min_opt: &mut Option<(i32, i32)>,
    seen: &mut HashSet<(i32,i32)>,
) {
    // println!("current = {:?}",current);
    // let _ = io::stdout().flush();
    if seen.contains(&current){
        return
    }
    seen.insert(current);
    if current.0 > 100 || current.1 > 100 {
        return;
    }
    if current.0 * claw_machine.a.0 + current.1 * claw_machine.b.0 > claw_machine.p.0
        || current.0 * claw_machine.a.1 + current.1 * claw_machine.b.1 > claw_machine.p.1
    {
        return;
    }
    if current.0 * claw_machine.a.0 + current.1 * claw_machine.b.0 == claw_machine.p.0
        && current.0 * claw_machine.a.1 + current.1 * claw_machine.b.1 == claw_machine.p.1
    {
        println!("found sol >> {current:?}");
        match &min_opt {
            None => *min_opt = Some(current),
            Some(min) => {
                if min.0 * TOKENS.0 + min.0 * TOKENS.1 < current.0 * TOKENS.0 + current.1 * TOKENS.1
                {
                    *min_opt = Some(current)
                }
            }
        }
        return;
    }
    recurse_find_minimum_cost((current.0 + 1, current.1), claw_machine, min_opt,seen);
    recurse_find_minimum_cost((current.0, current.1 + 1), claw_machine, min_opt,seen);
}
#[derive(Clone, Debug)]
struct ClawMachine {
    a: (i32, i32),
    b: (i32, i32),
    p: (i32, i32),
}
