use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign, Mul, Sub},
};

const PREFIX: [char; 3] = ['p', 'v', '='];
const Y_MAX: i32 = 103;
const X_MAX: i32 = 101;
// const Y_MAX: i32 = 7;
// const X_MAX: i32 = 11;
const TIME_ELAPSED: i32 = 100;

fn main() {
    // let file = File::open("./examples/day-14-part-01/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-14-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut robots: Vec<Robot> = Vec::new();
    let mut matrix: Vec<Vec<Option<i32>>> = (0..Y_MAX)
        .map(|_| (0..X_MAX).map(|_| None).collect())
        .collect();

    for line in buff.lines() {
        let Ok(line) = line else {
            unreachable!();
        };
        if line.is_empty() {
            continue;
        }
        let mut line_split = line.split(" ");
        let (Some(pos), Some(vel), None) = (
            line_split.next().map(|x| x.trim_start_matches(PREFIX)),
            line_split.next().map(|x| x.trim_start_matches(PREFIX)),
            line_split.next(),
        ) else {
            unreachable!();
        };
        let (mut pos_split, mut vel_split) = (pos.split(","), vel.split(","));
        let (Some(px), Some(py), None) = (
            pos_split.next().and_then(|x| x.parse::<i32>().ok()),
            pos_split.next().and_then(|x| x.parse::<i32>().ok()),
            pos_split.next(),
        ) else {
            panic!("error parsing data");
        };
        let (Some(vx), Some(vy), None) = (
            vel_split.next().and_then(|x| x.parse::<i32>().ok()),
            vel_split.next().and_then(|x| x.parse::<i32>().ok()),
            vel_split.next(),
        ) else {
            panic!("error parsing data");
        };
        robots.push(Robot {
            p: Point { x: px, y: py },
            v: Point { x: vx, y: vy },
        });
    }
    let mut quadrants: [i32; 4] = [0, 0, 0, 0];
    for robot in robots.iter_mut() {
        robot.p.x = (robot.p.x + robot.v.x * TIME_ELAPSED) % X_MAX;
        if robot.p.x < 0 {
            robot.p.x += X_MAX;
        }
        robot.p.y = (robot.p.y + robot.v.y * TIME_ELAPSED) % Y_MAX;
        if robot.p.y < 0 {
            robot.p.y += Y_MAX;
        }
        println!("x>> {}, y>> {}", robot.p.x, robot.p.y);
        match &matrix[robot.p.y as usize][robot.p.x as usize] {
            None => matrix[robot.p.y as usize][robot.p.x as usize] = Some(1),
            Some(i) => matrix[robot.p.y as usize][robot.p.x as usize] = Some(i + 1),
        }
        if robot.p.x >= 0 && robot.p.x < X_MAX / 2 && robot.p.y >= 0 && robot.p.y < Y_MAX / 2 {
            quadrants[0] += 1;
        }
        if robot.p.x >= 0 && robot.p.x < X_MAX / 2 && robot.p.y > (Y_MAX / 2) && robot.p.y < Y_MAX {
            quadrants[1] += 1;
        }
        if robot.p.x > (X_MAX / 2) && robot.p.x < X_MAX && robot.p.y >= 0 && robot.p.y < Y_MAX / 2 {
            quadrants[2] += 1;
        }
        if robot.p.x > (X_MAX / 2)
            && robot.p.x < X_MAX
            && robot.p.y > (Y_MAX / 2)
            && robot.p.y < Y_MAX
        {
            quadrants[3] += 1;
        }
    }
    print_matrix(&matrix);
    println!("{quadrants:?}");
    let acc = quadrants.iter().fold(1, |a, x| a * *x);
    println!("Result -> {}", acc);
}
fn print_matrix(m: &[Vec<Option<i32>>]) {
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
struct Robot {
    p: Point<i32>,
    v: Point<i32>,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl Point<i32> {
    fn is_inside_mat(&self, x_len: i32, y_len: i32) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < x_len && self.y < y_len
    }
}
impl Sub for Point<i32> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Add for Point<i32> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Point<i32> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}
impl Mul<i32> for Point<i32> {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
