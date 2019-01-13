use std::io::{self, BufRead};
use std::collections::BTreeSet;

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let line1 = iter.next().unwrap().unwrap();
    let line2 = iter.next().unwrap().unwrap();
    let v: Vec<u32> = line1.split_whitespace().map(|num| num.parse().unwrap()).collect();
    let W = v[0];
    let mut v: Vec<u32> = line2.split_whitespace().map(|num| num.parse().unwrap()).collect();
    v.insert(0, 0);
    v.push(W);

    let mut res = BTreeSet::new();
    let len = v.len();
    for i in 0..len-1 {
        for j in i+1..len {
            res.insert(v[j]-v[i]);
        }
    }
    let result: Vec<_> = res.into_iter().map(|num| num.to_string()).collect();
    println!("{}", result.join(" "));
}
