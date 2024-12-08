use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Sub},
};

fn main() {
    // let file = File::open("./examples/day-08-part-02/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-08-part-02/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut matrix_antenna: Vec<Vec<Option<char>>> = Vec::new();
    let mut map_antenna: HashMap<char, Vec<Point<i32>>> = HashMap::new();
    let mut node_set: HashSet<Point<i32>> = HashSet::new();
    for (i, line) in buff.lines().enumerate() {
        let Ok(line) = line else {
            unreachable!();
        };
        let mut v_line: Vec<Option<char>> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => v_line.push(None),
                c => {
                    map_antenna
                        .entry(c)
                        .and_modify(|x| {
                            x.push(Point {
                                x: j as i32,
                                y: i as i32,
                            })
                        })
                        .or_insert(vec![Point {
                            x: j as i32,
                            y: i as i32,
                        }]);
                    v_line.push(Some(c));
                }
            }
        }
        matrix_antenna.push(v_line);
    }
    let x_len = matrix_antenna[0].len() as i32;
    let y_len = matrix_antenna.len() as i32;
    for (_, v) in map_antenna.iter() {
        for i in 0..v.len() - 1 {
            for j in i + 1..v.len() {
                let dif = (v[j] - v[i]).primitive_point();
                let mut n1 = v[i];
                while n1.is_inside_mat(x_len, y_len) {
                    node_set.insert(n1);
                    matrix_antenna[n1.y as usize][n1.x as usize] = Some('#');
                    n1 = n1 + dif;
                }
                n1 = v[i];
                while n1.is_inside_mat(x_len, y_len) {
                    node_set.insert(n1);
                    matrix_antenna[n1.y as usize][n1.x as usize] = Some('#');
                    n1 = n1 - dif;
                }
            }
        }
    }

    print_matix(&matrix_antenna);
    println!("Result -> {}", node_set.len());
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
    fn primitive_point(self) -> Self {
        let v_gcd = gcd(self.x, self.y);
        Self {
            x: self.x / v_gcd,
            y: self.y / v_gcd,
        }
    }
}
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
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
