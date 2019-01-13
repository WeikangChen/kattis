use std::io::{self, BufRead};
use std::collections::HashSet;

fn find_neighs(catan: &Vec<i32>, i: usize, lvl: usize, nxt: usize) -> HashSet<usize>
{
    let mut res = HashSet::new();
    let pre = nxt - lvl * 6;
    let idx = i - pre;
    let div = idx / lvl;
    let rem = idx % lvl;
    res.insert(catan[pre + div * lvl] as usize);
    if rem > 0 {
        res.insert(catan[pre + (div -1) * lvl+ rem] as usize);
    }
    println!("i={} idx={} div={} rem={}", i, idx, div, rem);
    res
}

fn find_min(cnts: &Vec<i32>, neighs: &HashSet<usize>) -> usize
{
    let mut cands = Vec::new();
    for i in 1..=5 {
        if !neighs.contains(&i) {
            cands.push(i); 
        }
    }
    let mut r = 1;
    for c in cands {
        if cnts[c] < cnts[r] {
            r = c
        }
    }
    r
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let _ = iter.next();

    static len: usize = 10;
    let mut catan = vec![0; len];
    let mut cnts =vec![0; 6];
    catan[0] = 1;
    cnts[1] = 1;
    let mut prev = 1;
    let mut lvl: usize = 1;
    let mut nxt: usize = 6;
    for i in 1..len {
        let mut neighs = find_neighs(&catan, i, lvl, nxt);
        neighs.insert(prev);
        let cur =  find_min(&cnts, &neighs);
        cnts[cur] += 1;
        catan[i] = cur as i32;
        prev = cur;
        if i == nxt {
            lvl += 1;
            nxt += lvl * 6;
        }
    }
    for line in iter.map(|l| l.unwrap()) {
        let i: usize = line.parse().unwrap();
        println!("{}", catan[i-1]);
    }

}
