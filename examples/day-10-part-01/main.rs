use std::{
    collections::HashSet, fs::File, io::{BufRead, BufReader}, ops::{Add, Sub}
};

fn main() {
    // let file = File::open("./examples/day-10-part-01/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-10-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut matrix_h: Vec<Vec<i32>> = Vec::new();
    let mut zeros: Vec<Point<i32>> = Vec::new();
    for (i, line) in buff.lines().enumerate() {
        let Ok(line) = line else {
            unreachable!();
        };
        let mut v_line: Vec<i32> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let n = c.to_digit(10).expect("char is not a number") as i32;
            if n == 0 {
                zeros.push(Point {
                    x: j as i32,
                    y: i as i32,
                });
            }
            v_line.push(n);
        }
        matrix_h.push(v_line);
    }
    let mut acc = 0usize;
    for z in zeros.iter() {
        let mut nines_set: HashSet<Point<i32>> = HashSet::new();
        recurse_find_distinct_nine(-1, *z, &matrix_h,&mut nines_set);
        acc += nines_set.len();
        // println!("{}",nines_set.len());
    }

    println!("Result -> {}", acc);
}
const DIR: [Point<i32>;4] = [Point{x:0,y:1},Point{x:0,y:-1},Point{x:1,y:0},Point{x:-1,y:0}];
fn recurse_find_distinct_nine(last_n:i32,p: Point<i32>, m: &[Vec<i32>], nines_set: &mut HashSet<Point<i32>>){
    if !p.is_inside_mat(m[0].len() as i32, m.len() as i32) {
        return
    }
    let n = m[p.y as usize][p.x as usize] ;
    if last_n+1 != n {
        return
    }
    if n == 9 {
        nines_set.insert(p);
        return
    }
    for p_delta in DIR.iter(){
        recurse_find_distinct_nine(n,p+*p_delta, m,nines_set);
    }
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
