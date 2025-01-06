use core::panic;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign, Sub},
};
// const MAT_DIM: usize = 7; //test
const MAT_DIM: usize = 71;
// const N_MAX: usize = 12; //test
const N_MAX: usize = 1024;

fn main() {
    // let file = File::open("./examples/day-18-part-01/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-18-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut points: Vec<Point<i64>> = Vec::new();
    let mut matrix: Vec<Vec<Option<char>>> = vec![vec![None; MAT_DIM]; MAT_DIM];
    for line in buff.lines() {
        let Ok(line) = line else {
            panic!("error reading line");
        };
        let mut line_split = line.split(",");
        let (Some(x), Some(y), None) = (
            line_split.next().and_then(|x| x.parse::<i64>().ok()),
            line_split.next().and_then(|x| x.parse::<i64>().ok()),
            line_split.next(),
        ) else {
            continue;
        };
        points.push(Point { x, y });
    }
    points
        .iter()
        .take(N_MAX)
        .for_each(|p| matrix[p.y as usize][p.x as usize] = Some('#'));
    let (res, path) = dijkstra(&matrix);
    for p in path.iter() {
        matrix[p.y as usize][p.x as usize] = Some('O');
    }
    print_matrix(&matrix);
    println!("Result -> {}", res);
}
fn dijkstra(m: &[Vec<Option<char>>]) -> (i64, Vec<Point<i64>>) {
    let start = Point::<i64> { x: 0, y: 0 };
    let end = Point::<i64> {
        x: MAT_DIM as i64 - 1,
        y: MAT_DIM as i64 - 1,
    };
    let mut heap: BinaryHeap<Reverse<(i64, Point<i64>)>> = BinaryHeap::new();
    let mut distances: HashMap<Point<i64>, i64> = HashMap::new();
    let mut parent_map: HashMap<Point<i64>, Point<i64>> = HashMap::new();
    distances.insert(start, 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((dist, current))) = heap.pop() {
        if current == end {
            let mut path = vec![];
            let mut current_point = end;
            // Backtrack from the end to start using parent_map
            while let Some(&parent) = parent_map.get(&current_point) {
                path.push(current_point);
                current_point = parent;
            }
            path.push(start); // Add the start point
            path.reverse(); // Reverse the path to get it from start to end
            return (dist, path);
        }

        for neighbor in current.neighbors() {
            if !neighbor.is_inside_mat(MAT_DIM as i64, MAT_DIM as i64) {
                continue;
            }
            if m[neighbor.y as usize][neighbor.x as usize] == Some('#') {
                continue;
            }

            let new_dist = dist + 1;
            if new_dist < *distances.get(&neighbor).unwrap_or(&i64::MAX) {
                distances.insert(neighbor, new_dist);
                heap.push(Reverse((new_dist, neighbor)));
                parent_map.insert(neighbor, current);
            }
        }
    }

    (i64::MAX, vec![]) // No path found
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
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl Ord for Point<i64> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

impl PartialOrd for Point<i64> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Point<i64> {
    fn is_inside_mat(&self, x_len: i64, y_len: i64) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < x_len && self.y < y_len
    }
    fn neighbors(&self) -> Vec<Point<i64>> {
        let mut result = Vec::new();
        if self.x > 0 {
            result.push(Point {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            result.push(Point {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.x + 1 < MAT_DIM as i64 {
            result.push(Point {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y + 1 < MAT_DIM as i64 {
            result.push(Point {
                x: self.x,
                y: self.y + 1,
            });
        }
        result
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
