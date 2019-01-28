use whiteread::parse_line;
use std::collections::VecDeque;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(p: (i32, i32)) -> Point {
        Point { x: p.0, y: p.1 }
    }

    fn man_distance(&self, rhs: &Point) -> i32 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }

    fn can_reach(&self, rhs: &Point) -> bool {
        self.man_distance(rhs) <= 1000
    }
}

fn solve(start: Point, mut points: Vec<Point>, target: Point) -> bool {
    let mut que = VecDeque::new();
    que.push_back(start);

    while let Some(cur) = que.pop_front() {
        if cur.can_reach(&target) {
            return true;
        }
        let (v1, v2) = points.into_iter().partition(|p| p.can_reach(&cur));
        que.extend(v1);
        points = v2;
    }
    false
}

fn main() {
    let n: usize = parse_line().unwrap();
    for _i in 0..n {
        let t: usize = parse_line().unwrap();
        let v: (i32, i32) = parse_line().unwrap();
        let start = Point::new(v);
        let mut points = Vec::new();
        for _j in 0..t {
            let v: (i32, i32) = parse_line().unwrap();
            let p = Point::new(v);
            points.push(p);
        }
        let v: (i32, i32) = parse_line().unwrap();
        let target = Point::new(v);

        let ans = solve(start, points, target);
        println!("{}", if ans { "happy" } else { "sad" });
    }
}
