use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let file = File::open("./examples/day-05-part-01/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-05-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut stop_rules = false;
    let mut m = HashMap::<i32, Vec<i32>>::new();
    let mut updates = Vec::<Vec<i32>>::new();
    let mut acc = 0i32;
    for line in buff.lines() {
        let Ok(line) = line else {
            unreachable!("error reading line");
        };
        if line.is_empty() {
            stop_rules = true;
            continue;
        }
        // println!("{line}");
        match &stop_rules {
            false => add_rule(&mut m, &line).expect("error processing rule"),
            true => updates.push(add_update(&line).expect("error processing update")),
        }
    }
    // println!("{m:?}");
    'outer: for update in updates.iter() {
        'mid: for i in 0..update.len() {
            let Some(vals) = m.get(&update[update.len() - 1 - i]) else {
                continue 'mid;
            };
            for before in update.iter().rev().skip(1+i) {
                if vals.contains(before) {
                    // println!("found invalid {before} cant be before {}",&update[update.len()-1-i]);
                    continue 'outer;
                }
            }
        }
        // println!("adding {} from {:?}",update[update.len()/2], update);
        acc += update[update.len()/2];
    }
    println!("Result -> {}", acc);
}
fn add_rule(m: &mut HashMap<i32, Vec<i32>>, s: &str) -> Result<(), String> {
    let mut s_split = s.split("|");
    let (Some(k), Some(v), None) = (
        s_split.next().and_then(|x| x.parse::<i32>().ok()),
        s_split.next().and_then(|x| x.parse::<i32>().ok()),
        s_split.next(),
    ) else {
        return Err(String::from("error parsing rule"));
    };
    m.entry(k).and_modify(|x| x.push(v)).or_insert(vec![v]);

    Ok(())
}
fn add_update(s: &str) -> Result<Vec<i32>, String> {
    s.split(",")
        .map(|x| {
            x.parse::<i32>()
                .map_err(|_| String::from("error parsing updates"))
        })
        .collect()
}
