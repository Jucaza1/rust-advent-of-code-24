use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let file = File::open("./examples/day-06-part-01/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-06-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut seen = HashSet::<Point<i32>>::new();
    let mut map_matrix = Vec::<Vec<Option<char>>>::new();
    let mut guard_pos: Point<i32> = Point { x: 0, y: 0 };
    let mut guard_dir: Dir = Dir::Up;
    for (i, line) in buff.lines().enumerate() {
        let Ok(line) = line else {
            unreachable!();
        };
        map_matrix.push(
            line.chars()
                .enumerate()
                .map(|(j, x)| match x {
                    '#' => Some('#'),
                    '^' => {
                        guard_pos.x = j as i32;
                        guard_pos.y = i as i32;
                        None
                    }
                    _ => None,
                })
                .collect(),
        )
    }
    let m_y_len = map_matrix.len() as i32;
    let m_x_len = map_matrix[0].len() as i32;
    loop  {
        if guard_pos.y < 0
            || guard_pos.x < 0
            || guard_pos.y > m_y_len - 1
            || guard_pos.x > m_x_len - 1
        {
            break;
        }
        // println!("{:?}", guard_pos);
        match (
            &map_matrix[guard_pos.y as usize][guard_pos.x as usize],
            &guard_dir,
        ) {
            (Some(_), Dir::Up) => {
                guard_pos.y += 1;
                guard_dir = Dir::Right;
            }
            (Some(_), Dir::Right) => {
                guard_pos.x -= 1;
                guard_dir = Dir::Down;
            }
            (Some(_), Dir::Down) => {
                guard_pos.y -= 1;
                guard_dir = Dir::Left;
            }
            (Some(_), Dir::Left) => {
                guard_pos.x += 1;
                guard_dir = Dir::Up;
            }
            (None, _) => (),
        }
        seen.insert(guard_pos.clone());
        match &guard_dir {
            Dir::Up => guard_pos.y -= 1,
            Dir::Right => guard_pos.x += 1,
            Dir::Down => guard_pos.y += 1,
            Dir::Left => guard_pos.x -= 1,
        }
    }

    println!("Result -> {}", seen.len());
}
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Point<T> {
    x: T,
    y: T,
}
enum Dir {
    Up,
    Right,
    Down,
    Left,
}
