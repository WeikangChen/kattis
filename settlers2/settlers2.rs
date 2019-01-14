use std::io::{self, BufRead};
use std::collections::HashSet;

fn find_nbrs(catan: &Vec<usize>, 
             cur_idx: usize, 
             from_idx: usize, 
             lvl: usize) 
             -> HashSet<usize>
{
    let idx = cur_idx - from_idx;
    let div = idx / lvl;
    let rem = idx % lvl;
    let last_lvl = lvl - 1;
    let last_from = from_idx - last_lvl * 6;
    let last_nbr = last_from + last_lvl * div + rem;

    let mut nbrs = HashSet::new();
    nbrs.insert(catan[cur_idx-1]);
    nbrs.insert(catan[last_nbr]);
    if cur_idx > from_idx + 1 && rem > 0 {
        nbrs.insert(catan[last_nbr-1]);
    }
    if cur_idx == from_idx + lvl * 6 {
        nbrs.insert(catan[from_idx+1]);
    } 
/*
    println!("{}:lvl={} from_idx={} idx={} div={} rem={}",
             cur_idx, lvl, from_idx, idx, div, rem);
    println!("\t last_from={} last_nbrs={}, nbrs_catan={:?}", 
             last_from, last_nbr, nbrs);
*/
    nbrs
}

fn find_catan(cnts: &Vec<usize>, nbrs: &HashSet<usize>) -> usize
{
    let candidates = (1..6).filter(|x| !nbrs.contains(x))
                           .collect::<Vec<usize>>();

    // println!("{:?}", candidates);
    let (_min, res) = candidates.into_iter()
                                .map(|i| (cnts[i], i))
                                .min().unwrap();

    res
    /* same as 
    let mut r = candidates[0];
    for &c in &candidates[1..] {
        if cnts[c] < cnts[r] {
            r = c;
        }
    }
    r
    */
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let _ = iter.next();
    static L: usize = 10000;
    let mut catan = vec![0; L];
    let mut cnts =vec![0; 6];
    catan[0] = 1;
    cnts[1] = 1;

    let mut lvl: usize = 1;
    let mut cur_from: usize = 0;
    let mut nxt_from: usize = 6;
    for cur in 1..L {
        let mut nbrs = find_nbrs(&catan, cur, cur_from, lvl);
        let res = find_catan(&cnts, &nbrs);
        // println!("\t catan[{}]={}", cur, res);
        catan[cur] = res;
        cnts[res] += 1;
        if cur == nxt_from {
            lvl += 1;
            cur_from = nxt_from;
            nxt_from += lvl * 6;
        }
    }
    for line in iter.map(|l| l.unwrap()) {
        let i: usize = line.parse().unwrap();
        println!("{}", catan[i-1]);
    }

}
