use core::panic;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign, Mul, Sub},
};

const END: char = 'E';
const START: char = 'S';
const WALL: char = '#';

fn main() {
    // let file = File::open("./examples/day-16-part-01/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-16-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut matrix: Vec<Vec<Option<char>>> = Vec::new();
    let mut initial_pos: Point<i64> = Point { x: 0, y: 0 };
    for (i, line) in buff.lines().enumerate() {
        let Ok(line) = line else {
            panic!("error reading line");
        };
        if line.is_empty() {
            continue;
        }
        let mut row: Vec<Option<char>> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                WALL => {
                    row.push(Some(c));
                }
                END => {
                    row.push(Some(c));
                }
                START => {
                    initial_pos = Point {
                        x: j as i64,
                        y: i as i64,
                    };
                    row.push(Some(c));
                }
                _ => {
                    row.push(None);
                }
            }
        }
        matrix.push(row)
    }
    print_matrix(&matrix);
    let pos = initial_pos;
    let mut scores = HashMap::<i64, Vec<Point<i64>>>::new();
    let mut seen: HashMap<Point<i64>,i64> = HashMap::new();
    let path: Vec<Point<i64>> = Vec::new();
    let mut min_score = i64::MAX;

    let _ = walk_maze(
        Direction::Right,
        pos,
        &matrix,
        &mut seen,
        &mut scores,
        0,
        path,
        &mut min_score,
    );
    let mut min_score = i64::MAX;
    let mut min_path = Vec::<Point<i64>>::new();
    for (score, path) in scores.iter() {
        if min_score > *score {
            min_score = *score;
            min_path = path.clone();
        }
    }
    for point in min_path.iter() {
        matrix[point.y as usize][point.x as usize] = Some('O')
    }
    print_matrix(&matrix);
    println!("Result -> {}", min_score);
}
fn walk_maze(
    mut dir: Direction,
    pos: Point<i64>,
    m: &[Vec<Option<char>>],
    seen: &mut HashMap<Point<i64>, i64>,
    score_v: &mut HashMap<i64, Vec<Point<i64>>>,
    score_path: i64,
    mut path: Vec<Point<i64>>,
    min_score: &mut i64,
) -> State {
    let cell = m[pos.y as usize][pos.x as usize];
    match cell {
        Some('E') => {
            score_v.insert(score_path, path);
            *min_score = score_path;
            return State::End;
        }
        Some('#') => return State::Stuck,
        _ => (),
    }
    match seen.get_mut(&pos) {
        Some(seen_score) => {
            if *seen_score <= score_path {
                return State::Stuck;
            }
            *seen_score = score_path;
        }
        None => {
            seen.insert(pos, score_path);
        }
    };
    path.push(pos);
    let first = walk_maze(
        dir,
        pos + next_delta(dir),
        m,
        seen,
        score_v,
        score_path + 1,
        path.clone(),
        min_score,
    );
    dir = next_dir(dir);
    let second = walk_maze(
        dir,
        pos + next_delta(dir),
        m,
        seen,
        score_v,
        score_path + 1001,
        path.clone(),
        min_score,
    );
    dir = next_dir(dir);
    let third = walk_maze(
        dir,
        pos + next_delta(dir),
        m,
        seen,
        score_v,
        score_path + 2001,
        path.clone(),
        min_score,
    );
    dir = next_dir(dir);
    let fourth = walk_maze(
        dir,
        pos + next_delta(dir),
        m,
        seen,
        score_v,
        score_path + 1001,
        path.clone(),
        min_score,
    );
    if let (State::Stuck, State::Stuck, State::Stuck, State::Stuck) = (first, second, third, fourth)
    {
        path.pop();
        return State::Stuck;
    }
    State::Walkable
}
fn print_matrix(m: &[Vec<Option<char>>]) {
    for i in 0..m.len() {
        for j in 0..m[0].len() {
            match m[i][j] {
                None => print!("."),
                Some(x) => print!("{x}"),
            }
        }
        println!();
    }
}
enum State {
    Walkable,
    End,
    Stuck,
}

const UP: Point<i64> = Point::<i64> { x: 0, y: -1 };
const RIGHT: Point<i64> = Point::<i64> { x: 1, y: 0 };
const DOWN: Point<i64> = Point::<i64> { x: 0, y: 1 };
const LEFT: Point<i64> = Point::<i64> { x: -1, y: 0 };
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
fn next_dir(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}
fn next_delta(dir: Direction) -> Point<i64> {
    match dir {
        Direction::Up => UP,
        Direction::Right => RIGHT,
        Direction::Down => DOWN,
        Direction::Left => LEFT,
    }
}
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl Point<i64> {
    fn is_inside_mat(&self, x_len: i64, y_len: i64) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < x_len && self.y < y_len
    }
}
impl Sub for Point<i64> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Add for Point<i64> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Point<i64> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}
impl Mul<i64> for Point<i64> {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
