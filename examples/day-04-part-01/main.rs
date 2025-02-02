use std::{
    fs::File,
    io::{BufRead, BufReader},
};
const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

fn main() {
    // let file = File::open("./examples/day-04-part-01/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-04-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in buff.lines() {
        matrix.push(line.expect("error reading the line").chars().collect())
    }
    let acc = process_matrix(&matrix);
    println!("Result -> {}", acc);
}
fn process_matrix(m: &[Vec<char>]) -> i32 {
    let mut acc = 0i32;
    for row in 0..m.len() {
        for col in 0..m[0].len() {
            if WORD[0] == m[row][col] {
                acc += find_char(m, Dir::All, row as i32, col as i32, 0);
            }
        }
    }
    acc
}
fn find_char(m: &[Vec<char>], dir: Dir, row: i32, col: i32, indx: usize) -> i32 {
    let mut acc = 0i32;
    if indx == WORD.len() {
        return 1;
    }
    if row < 0 || col < 0 || row == m.len() as i32 || col == m[0].len() as i32 {
        return 0;
    }
    if m[row as usize][col as usize] != WORD[indx] {
        return 0
    }
    match &dir {
        Dir::All => {
            acc += find_char(m, Dir::TopLeft, row - 1, col - 1, 1);
            acc += find_char(m, Dir::Top, row - 1, col, 1);
            acc += find_char(m, Dir::TopRight, row - 1, col + 1, 1);
            acc += find_char(m, Dir::Left, row, col - 1, 1);
            acc += find_char(m, Dir::Right, row , col + 1, 1);
            acc += find_char(m, Dir::BotLeft, row + 1, col - 1, 1);
            acc += find_char(m, Dir::Bot, row + 1, col, 1);
            acc += find_char(m, Dir::BotRight, row + 1, col + 1, 1);
        }
        Dir::TopLeft => {
            return find_char(m, dir, row - 1, col - 1, indx + 1);
        }
        Dir::Top => {
            return find_char(m, dir, row - 1, col, indx + 1);
        }
        Dir::TopRight => {
            return find_char(m, dir, row - 1, col + 1, indx + 1);
        }
        Dir::Left => {
            return find_char(m, dir, row, col - 1, indx + 1);
        }
        Dir::Right => {
            return find_char(m, dir, row, col + 1, indx + 1);
        }
        Dir::BotLeft => {
            return find_char(m, dir, row + 1, col - 1, indx + 1);
        }
        Dir::Bot => {
            return find_char(m, dir, row + 1, col, indx + 1);
        }
        Dir::BotRight => {
            return find_char(m, dir, row + 1, col + 1, indx + 1);
        }
    }
    acc
}
enum Dir {
    All,
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BotLeft,
    Bot,
    BotRight,
}
