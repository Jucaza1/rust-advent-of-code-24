use std::{
    fs::File,
    io::{BufRead, BufReader},
};
const WORD: [char; 3] = ['A', 'M', 'S'];

fn main() {
    // let file = File::open("./examples/day-04-part-02/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-04-part-02/input.txt").expect("file not found");
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
    for row in 1..m.len()-1 {
        for col in 1..m[0].len()-1 {
            if WORD[0] == m[row][col] && 2 == find_char(m, Dir::All, row as i32, col as i32, 0) {
                acc += 1
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
        return 0;
    }
    match &dir {
        Dir::All => {
            acc += find_char(m, Dir::TopLeft, row - 1, col - 1, 1);
            acc += find_char(m, Dir::TopRight, row - 1, col + 1, 1);
            acc += find_char(m, Dir::BotLeft, row + 1, col - 1, 1);
            acc += find_char(m, Dir::BotRight, row + 1, col + 1, 1);
        }
        Dir::TopLeft => {
            return find_char(m, dir, row + 2, col + 2, indx + 1);
        }
        Dir::TopRight => {
            return find_char(m, dir, row + 2, col - 2, indx + 1);
        }
        Dir::BotLeft => {
            return find_char(m, dir, row - 2, col + 2, indx + 1);
        }
        Dir::BotRight => {
            return find_char(m, dir, row - 2, col - 2, indx + 1);
        }
    }
    acc
}
enum Dir {
    All,
    TopLeft,
    TopRight,
    BotLeft,
    BotRight,
}
