use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Sub},
};

fn main() {
    // let file = File::open("./examples/day-12-part-02/input-test2.txt").expect("file not found");
    // let file = File::open("./examples/day-12-part-02/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-12-part-02/input.txt").expect("file not found");
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
            if let Some((area, corners)) = recurse_count_area_corners(*c, *p, &matrix_char, &mut seen)
            {
                if area != 0 && corners != 0 {
                    let price = area * corners;
                    acc += price;
                    println!("- A region of {c} plants with price {area} * {corners} = {price}.");
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
fn recurse_count_area_corners(
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
    let mut corners_acc = count_corners(c, p, m);
    for p_delta in DIR.iter() {
        if let Some((area, corners)) = recurse_count_area_corners(c, p + *p_delta, m, seen) {
            corners_acc += corners;
            area_acc += area;
        }
    }
    Some((area_acc, corners_acc))
}
const TOP_LEFT: Point<i32> = Point { x: -1, y: -1 };
const TOP_RIGHT: Point<i32> = Point { x: 1, y: -1 };
const BOT_LEFT: Point<i32> = Point { x: -1, y: 1 };
const BOT_RIGHT: Point<i32> = Point { x: 1, y: 1 };
const TOP: Point<i32> = Point { x: 0, y: -1 };
const RIGHT: Point<i32> = Point { x: 1, y: 0 };
const BOT: Point<i32> = Point { x: 0, y: 1 };
const LEFT: Point<i32> = Point { x: -1, y: 0 };
fn count_corners(c: char, p: Point<i32>, m: &[Vec<char>]) -> i32 {
    let (x_len,y_len) = (m[0].len() as i32,m.len() as i32);
    let mut corners = 0i32;
    //outer corners
    if
    (!(p+TOP).is_inside_mat(x_len, y_len)|| m[(p+TOP).y as usize][(p+TOP).x as usize] != c)
    &&(!(p+LEFT).is_inside_mat(x_len, y_len)|| m[(p+LEFT).y as usize][(p+LEFT).x as usize] != c)
    {corners +=1;}
    if
    (!(p+TOP).is_inside_mat(x_len, y_len)|| m[(p+TOP).y as usize][(p+TOP).x as usize] != c)
    &&(!(p+RIGHT).is_inside_mat(x_len, y_len)|| m[(p+RIGHT).y as usize][(p+RIGHT).x as usize] != c)
    {corners +=1;}
    if
    (!(p+BOT).is_inside_mat(x_len, y_len)|| m[(p+BOT).y as usize][(p+BOT).x as usize] != c)
    &&(!(p+RIGHT).is_inside_mat(x_len, y_len)|| m[(p+RIGHT).y as usize][(p+RIGHT).x as usize] != c)
    {corners +=1;}
    if
    (!(p+BOT).is_inside_mat(x_len, y_len)|| m[(p+BOT).y as usize][(p+BOT).x as usize] != c)
    &&(!(p+LEFT).is_inside_mat(x_len, y_len)|| m[(p+LEFT).y as usize][(p+LEFT).x as usize] != c)
    {corners +=1;}
    //inner corners
    if
    ((p+BOT).is_inside_mat(x_len, y_len)&& m[(p+BOT).y as usize][(p+BOT).x as usize] == c)
    &&((p+LEFT).is_inside_mat(x_len, y_len)&& m[(p+LEFT).y as usize][(p+LEFT).x as usize] == c)
    &&(!(p+BOT_LEFT).is_inside_mat(x_len, y_len)|| m[(p+BOT_LEFT).y as usize][(p+BOT_LEFT).x as usize] != c)
    {corners +=1;}
    if
    ((p+TOP).is_inside_mat(x_len, y_len)&& m[(p+TOP).y as usize][(p+TOP).x as usize] == c)
    &&((p+LEFT).is_inside_mat(x_len, y_len)&& m[(p+LEFT).y as usize][(p+LEFT).x as usize] == c)
    &&(!(p+TOP_LEFT).is_inside_mat(x_len, y_len)|| m[(p+TOP_LEFT).y as usize][(p+TOP_LEFT).x as usize] != c)
    {corners +=1;}
    if
    ((p+TOP).is_inside_mat(x_len, y_len)&& m[(p+TOP).y as usize][(p+TOP).x as usize] == c)
    &&((p+RIGHT).is_inside_mat(x_len, y_len)&& m[(p+RIGHT).y as usize][(p+RIGHT).x as usize] == c)
    &&(!(p+TOP_RIGHT).is_inside_mat(x_len, y_len)|| m[(p+TOP_RIGHT).y as usize][(p+TOP_RIGHT).x as usize] != c)
    {corners +=1;}
    if
    ((p+BOT).is_inside_mat(x_len, y_len)&& m[(p+BOT).y as usize][(p+BOT).x as usize] == c)
    &&((p+RIGHT).is_inside_mat(x_len, y_len)&& m[(p+RIGHT).y as usize][(p+RIGHT).x as usize] == c)
    &&(!(p+BOT_RIGHT).is_inside_mat(x_len, y_len)|| m[(p+BOT_RIGHT).y as usize][(p+BOT_RIGHT).x as usize] != c)
    {corners +=1;}
    corners
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
