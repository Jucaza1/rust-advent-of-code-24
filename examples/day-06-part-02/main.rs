use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let file = File::open("./examples/day-06-part-02/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-06-part-02/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut seen = HashMap::<Point<i32>, Vec<Dir>>::new();
    let mut map_matrix = Vec::<Vec<Option<char>>>::new();
    let mut added_obstacles: HashSet<Point<i32>> = HashSet::new();
    let mut current_obstacles: HashSet<Point<i32>> = HashSet::new();
    let mut guard_pos_ini: Point<i32> = Point { x: 0, y: 0 };
    let mut guard_pos: Point<i32>;
    let mut guard_dir: Dir = Dir::Up;
    for (i, line) in buff.lines().enumerate() {
        let Ok(line) = line else {
            unreachable!();
        };
        map_matrix.push(
            line.chars()
                .enumerate()
                .map(|(j, x)| match x {
                    '#' => {
                        current_obstacles.insert(Point {
                            x: j as i32,
                            y: i as i32,
                        });
                        Some('#')
                    }
                    '^' => {
                        guard_pos_ini.x = j as i32;
                        guard_pos_ini.y = i as i32;
                        Some('^')
                    }
                    _ => None,
                })
                .collect(),
        )
    }
    let m_y_len = map_matrix.len() as i32;
    let m_x_len = map_matrix[0].len() as i32;
    guard_pos = guard_pos_ini.clone();
    let mut looping = false;
    loop {
        if guard_pos.y < 0
            || guard_pos.x < 0
            || guard_pos.y > m_y_len - 1
            || guard_pos.x > m_x_len - 1
        {
            break;
        }
        match (
            &map_matrix[guard_pos.y as usize][guard_pos.x as usize],
            &guard_dir,
        ) {
            (Some('#'), Dir::Up) => {
                guard_pos.y += 1;
                guard_dir = Dir::Right;
            }
            (Some('#'), Dir::Right) => {
                guard_pos.x -= 1;
                guard_dir = Dir::Down;
            }
            (Some('#'), Dir::Down) => {
                guard_pos.y -= 1;
                guard_dir = Dir::Left;
            }
            (Some('#'), Dir::Left) => {
                guard_pos.x += 1;
                guard_dir = Dir::Up;
            }
            (_, _) => (),
        }
        let possible_obstacle: Point<i32> = match &guard_dir {
            Dir::Up => Point {
                x: guard_pos.x,
                y: guard_pos.y - 1,
            },
            Dir::Right => Point {
                x: guard_pos.x + 1,
                y: guard_pos.y,
            },
            Dir::Down => Point {
                x: guard_pos.x,
                y: guard_pos.y + 1,
            },
            Dir::Left => Point {
                x: guard_pos.x - 1,
                y: guard_pos.y,
            },
        };
        if possible_obstacle != guard_pos_ini
            && !current_obstacles.contains(&possible_obstacle)
            && !seen.contains_key(&possible_obstacle)
            && !(possible_obstacle.y < 0
            || possible_obstacle.x < 0
            || possible_obstacle.y > m_y_len - 1
            || possible_obstacle.x > m_x_len - 1)
        {
            let mut mat_with_obstacle = map_matrix.clone();
            mat_with_obstacle[possible_obstacle.y as usize][possible_obstacle.x as usize] =
                Some('O');
            if stays_in_loop(
                guard_pos.clone(),
                guard_dir.clone(),
                &seen,
                mat_with_obstacle.clone(),
            ) {
                // print_matix(&mat_with_obstacle);
                // println!("---");
                added_obstacles.insert(possible_obstacle);
            }
        }
        seen.entry(guard_pos.clone())
            .and_modify(|v| {
                if !v.contains(&guard_dir) {
                    v.push(guard_dir.clone());
                }
                else{
                    looping = true;
                    println!("The guard is already in a loop");
                }
            })
            .or_insert(vec![guard_dir.clone()]);
        match &guard_dir {
            Dir::Up => guard_pos.y -= 1,
            Dir::Right => guard_pos.x += 1,
            Dir::Down => guard_pos.y += 1,
            Dir::Left => guard_pos.x -= 1,
        }
    }

    println!("Result -> {}", added_obstacles.len());
}
fn print_matix(m: &[Vec<Option<char>>]) {
    for row in m.iter() {
        for col in row.iter() {
            match col {
                Some(x) => print!("{x}"),
                None => print!("."),
            }
        }
        println!();
    }
}
fn stays_in_loop(
    mut guard_pos: Point<i32>,
    mut guard_dir: Dir,
    seen: &HashMap<Point<i32>, Vec<Dir>>,
    map_matrix: Vec<Vec<Option<char>>>,
) -> bool {
    let mut imaginary_seen: HashMap<Point<i32>, Vec<Dir>> = seen.clone();
    let m_y_len = map_matrix.len() as i32;
    let m_x_len = map_matrix[0].len() as i32;
    let mut found = false;
    loop {
        if guard_pos.y < 0
            || guard_pos.x < 0
            || guard_pos.y > m_y_len - 1
            || guard_pos.x > m_x_len - 1
        {
            return false;
        }
        match (
            &map_matrix[guard_pos.y as usize][guard_pos.x as usize],
            &guard_dir,
        ) {
            (Some('#')|Some('O'), Dir::Up) => {
                guard_pos.y += 1;
                guard_dir = Dir::Right;
            }
            (Some('#')|Some('O'), Dir::Right) => {
                guard_pos.x -= 1;
                guard_dir = Dir::Down;
            }
            (Some('#')|Some('O'), Dir::Down) => {
                guard_pos.y -= 1;
                guard_dir = Dir::Left;
            }
            (Some('#')|Some('O'), Dir::Left) => {
                guard_pos.x += 1;
                guard_dir = Dir::Up;
            }
            (_, _) => (),
        }
        imaginary_seen
            .entry(guard_pos.clone())
            .and_modify(|v| {
                if !v.contains(&guard_dir) {
                    v.push(guard_dir.clone());
                } else {
                    found = true;
                }
            })
            .or_insert(vec![guard_dir.clone()]);
        if found {
            return true;
        }
        match &guard_dir {
            Dir::Up => guard_pos.y -= 1,
            Dir::Right => guard_pos.x += 1,
            Dir::Down => guard_pos.y += 1,
            Dir::Left => guard_pos.x -= 1,
        }
    }
}
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Point<T> {
    x: T,
    y: T,
}
#[derive(Eq, PartialEq, Clone, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}
