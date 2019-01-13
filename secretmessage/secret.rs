use std::io::{self, BufRead};
//use std::collections::BTreeSet;

fn solve(line: String) {
    let l = line.len();
    let n = (l as f64).sqrt().ceil() as usize;
    // println!("{} {}", l, s);

    let raw = line.as_bytes();
    let mut res = String::new();
    for c in 0..n {
        for r in (0..n).rev() {
            let p = r * n + c as usize;
            if p < l {
                res.push(raw[p] as char)
            }
        }
    }
    println!("{}", res)
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let _ = iter.next();

    for line in iter.map(|l| l.unwrap()) {
        // println!("{}", line);
        solve(line);
    }
}
