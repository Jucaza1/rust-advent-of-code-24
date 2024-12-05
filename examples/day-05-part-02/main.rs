use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let file = File::open("./examples/day-05-part-02/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-05-part-02/input.txt").expect("file not found");
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
    'outer: for update in updates.iter_mut() {
        let mut i = 0usize;
        let up_len = update.len();
        let mut fixed_update = false;
        'mid: while i < up_len {
            let Some(vals) = m.get(&update[update.len() - 1 - i]) else {
                i += 1;
                continue 'mid;
            };
            for j in 0..up_len-i {
                if vals.contains(&update[j]) {
                    if !fixed_update {
                        fixed_update = true;
                    }
                    println!("found invalid {} cant be before {}",update[j],update[update.len()-1-i]);
                    println!("from {:?}", update);
                    let temp = update[j];
                    update[j] = update[update.len() - 1 - i];
                    update[up_len - 1 - i] = temp;
                    println!("to   {:?}", update);
                    continue 'mid;
                }
            }
            i += 1;
        }
        if fixed_update {
            println!("adding {} from {:?}", update[update.len() / 2], update);
            acc += update[update.len() / 2];
            fixed_update = false;
        }
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
