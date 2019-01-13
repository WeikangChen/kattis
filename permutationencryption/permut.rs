use std::io::{self, BufRead};
//use std::collections::BTreeSet;

fn solve(n: usize, p: &[usize], dict: &[u8]) {
    let l = dict.len();
    let m = (l as f64 / n as f64).ceil() as usize;
    // println!("line={} n={} perm={:?}", line, n , p);
    // println!("len={} mod={} id={:?}", l, m, id);

    let mut res = String::new();
    for i in 0..m {
        for j in 0..n {
            let k = i * n + p[j] - 1;
            if k < l {
                res.push((dict[k]) as char);
            } else {
                res.push(' ')
            }
        }
    }
    println!("'{}'", res);
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();

    loop {
        let line1 = iter.next().unwrap().unwrap();
        // println!("{}", line1);
        let v: Vec<usize> = line1.split_whitespace().map(|num| num.parse().unwrap()).collect();
        if v[0] == 0 {
            break;
        }
        let line2 = iter.next().unwrap().unwrap();
        solve(v[0], &v[1..], line2.as_bytes());
    }
}
