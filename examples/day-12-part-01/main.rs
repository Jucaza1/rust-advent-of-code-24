use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Sub},
};

fn main() {
    // let file = File::open("./examples/day-12-part-01/input-test2.txt").expect("file not found");
    // let file = File::open("./examples/day-12-part-01/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-12-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut matrix_char: Vec<Vec<char>> = Vec::new();
    let mut map_char: HashMap<char, Vec<Point<i32>>> = HashMap::new();
    let mut seen: Vec<Vec<bool>> = Vec::new();
    for (i, line) in buff.lines().enumerate() {
        let Ok(line) = line else {
            unreachable!();
        };
        let mut v_line: Vec<char> = Vec::new();
        let mut seen_line: Vec<bool> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let p = Point {
                x: j as i32,
                y: i as i32,
            };
            map_char
                .entry(c)
                .and_modify(|x| x.push(p))
                .or_insert(vec![p]);
            v_line.push(c);
            seen_line.push(false);
        }
        matrix_char.push(v_line);
        seen.push(seen_line);
    }
    let mut acc = 0i32;
    for (c, points) in map_char.iter() {
        for p in points.iter() {
            if let Some((area, perim)) =
                recurse_count_area_perimeter(*c, *p, &matrix_char, &mut seen)
            {
                if area != 0 && perim != 0 {
                    let price = area * perim;
                    acc += price;
                    println!("- A region of {c} plants with price {area} * {perim} = {price}.");
                }
            }
        }
    }

    println!("Result -> {}", acc);
}
const DIR: [Point<i32>; 4] = [
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
];
fn recurse_count_area_perimeter(
    c: char,
    p: Point<i32>,
    m: &[Vec<char>],
    seen: &mut [Vec<bool>],
) -> Option<(i32, i32)> {
    if !p.is_inside_mat(m[0].len() as i32, m.len() as i32) {
        return None;
    }
    let c_m = m[p.y as usize][p.x as usize];
    if c_m != c {
        return None;
    }
    if seen[p.y as usize][p.x as usize] {
        return Some((0, 0));
    }
    seen[p.y as usize][p.x as usize] = true;
    let mut area_acc = 1i32;
    let mut perim_acc = 4i32;
    for p_delta in DIR.iter() {
        if let Some((area, perim)) = recurse_count_area_perimeter(c, p + *p_delta, m, seen) {
            perim_acc += -1 + perim;
            area_acc += area;
        }
    }
    Some((area_acc, perim_acc))
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
