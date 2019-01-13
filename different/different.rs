use std::io::{self, BufRead};

fn solve(a: i64, b: i64) {
    println!("{}", (a-b).abs());
}

fn main() {
    let stdin = io::stdin();
    let iter = stdin.lock().lines();
    
    for line in iter.map(|l| l.unwrap()) {
        let v: Vec<i64> = line.split_whitespace().map(
            |num| num.parse().unwrap()).collect();
        solve(v[0], v[1]);
    }
}
