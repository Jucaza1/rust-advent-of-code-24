use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign, Mul, Sub},
};

const BOX: char = 'O';
const START: char = '@';
const WALL: char = '#';

fn main() {
    // let file = File::open("./examples/day-15-part-02/input-test.txt").expect("file not found");
    // let file = File::open("./examples/day-15-part-02/input-test2.txt").expect("file not found");
    let file = File::open("./examples/day-15-part-02/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut matrix: Vec<Vec<Option<char>>> = Vec::new();
    let mut commands: Vec<Command> = Vec::new();
    let mut state_parse: StateParse = StateParse::Map;
    let mut initial_pos: Point<i64> = Point { x: 0, y: 0 };
    for (i, line) in buff.lines().enumerate() {
        let Ok(line) = line else {
            unreachable!();
        };
        if line.is_empty() {
            state_parse = StateParse::Mov;
            continue;
        }
        let mut row: Vec<Option<char>> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match (&state_parse, c) {
                (StateParse::Map, WALL) => {
                    row.push(Some('#'));
                    row.push(Some('#'));
                }
                (StateParse::Map, BOX) => {
                    row.push(Some('['));
                    row.push(Some(']'));
                }
                (StateParse::Map, START) => {
                    initial_pos = Point {
                        x: 2 * j as i64,
                        y: i as i64,
                    };
                    row.push(Some(c));
                    row.push(None);
                }
                (StateParse::Map, _) => {
                    row.push(None);
                    row.push(None);
                }
                (StateParse::Mov, c) => commands.push(match c {
                    '^' => Command::Up,
                    '>' => Command::Right,
                    'v' => Command::Down,
                    '<' => Command::Left,
                    _ => continue,
                }),
            }
        }
        if let StateParse::Map = state_parse {
            matrix.push(row)
        }
    }
    print_matrix(&matrix);
    let mut pos = initial_pos;
    for command in commands.iter() {
        let mov = match command {
            Command::Up => UP,
            Command::Right => RIGHT,
            Command::Down => DOWN,
            Command::Left => LEFT,
        };
        walk_or_push(mov, command, &mut pos, &mut matrix);
    }

    print_matrix(&matrix);
    println!("Result -> {}", sum_box_gps(&matrix));
}
fn check_forward(
    mov: Point<i64>,
    comm: &Command,
    pos: &mut Point<i64>,
    m: &mut [Vec<Option<char>>],
) -> bool {
    let cell = m[pos.y as usize][pos.x as usize];
    match (comm, &cell) {
        (_, None) => return true,
        (_, Some(WALL)) => return false,
        (Command::Up | Command::Down, Some('[')) => {
            if !(check_forward(mov, comm, &mut (*pos + mov).clone(), m)
                && check_forward(
                    mov,
                    comm,
                    &mut (*pos + mov + Point { x: 1, y: 0 }).clone(),
                    m,
                ))
            {
                return false;
            }
            return true;
        }
        (Command::Up | Command::Down, Some(']')) => {
            if !(check_forward(mov, comm, &mut (*pos + mov).clone(), m)
                && check_forward(
                    mov,
                    comm,
                    &mut (*pos + mov + Point { x: -1, y: 0 }).clone(),
                    m,
                ))
            {
                return false;
            }

            return true;
        }
        _ => {
            if !check_forward(mov, comm, &mut (*pos + mov).clone(), m) {
                return false;
            }
        }
    }
    true
}
fn walk_or_push(
    mov: Point<i64>,
    comm: &Command,
    pos: &mut Point<i64>,
    m: &mut [Vec<Option<char>>],
) -> bool {
    let cell = m[pos.y as usize][pos.x as usize];
    match (comm, &cell) {
        (_, None) => return true,
        (_, Some(WALL)) => return false,
        (Command::Up | Command::Down, Some('[')) => {
            if !(check_forward(mov, comm, &mut (*pos + mov).clone(), m)
                && check_forward(
                    mov,
                    comm,
                    &mut (*pos + mov + Point { x: 1, y: 0 }).clone(),
                    m,
                )
                && walk_or_push(mov, comm, &mut (*pos + mov).clone(), m)
                && walk_or_push(
                    mov,
                    comm,
                    &mut (*pos + mov + Point { x: 1, y: 0 }).clone(),
                    m,
                ))
            {
                return false;
            }
            let next = *pos + mov;
            m[next.y as usize][next.x as usize] = cell;
            m[pos.y as usize][pos.x as usize] = None;
            m[next.y as usize][(next.x + 1) as usize] = m[pos.y as usize][(pos.x + 1) as usize];
            m[pos.y as usize][(pos.x + 1) as usize] = None;
            *pos = next;

            return true;
        }
        (Command::Up | Command::Down, Some(']')) => {
            if !(check_forward(mov, comm, pos, m)
                && check_forward(
                    mov,
                    comm,
                    &mut (*pos + mov + Point { x: -1, y: 0 }).clone(),
                    m,
                )
                && walk_or_push(mov, comm, &mut (*pos + mov).clone(), m)
                && walk_or_push(
                    mov,
                    comm,
                    &mut (*pos + mov + Point { x: -1, y: 0 }).clone(),
                    m,
                ))
            {
                return false;
            }
            let next = *pos + mov;
            m[next.y as usize][next.x as usize] = cell;
            m[pos.y as usize][pos.x as usize] = None;
            m[next.y as usize][(next.x - 1) as usize] = m[pos.y as usize][(pos.x - 1) as usize];
            m[pos.y as usize][(pos.x - 1) as usize] = None;
            *pos = next;

            return true;
        }
        _ => {
            if !walk_or_push(mov, comm, &mut (*pos + mov).clone(), m) {
                return false;
            }
        }
    }
    let next = *pos + mov;
    m[next.y as usize][next.x as usize] = cell;
    m[pos.y as usize][pos.x as usize] = None;
    *pos = next;
    true
}
fn sum_box_gps(m: &[Vec<Option<char>>]) -> i64 {
    let mut acc = 0i64;
    for i in 0..m.len() {
        for j in 0..m[0].len() {
            if Some('[') == m[i][j] {
                acc += (100 * i + j) as i64
            }
        }
    }
    acc
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

const UP: Point<i64> = Point::<i64> { x: 0, y: -1 };
const RIGHT: Point<i64> = Point::<i64> { x: 1, y: 0 };
const DOWN: Point<i64> = Point::<i64> { x: 0, y: 1 };
const LEFT: Point<i64> = Point::<i64> { x: -1, y: 0 };
#[derive(Debug)]
enum Command {
    Up,
    Right,
    Down,
    Left,
}
enum StateParse {
    Map,
    Mov,
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
