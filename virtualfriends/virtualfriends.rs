use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
struct UnionFind {
    idmap: HashMap<String, usize>,
    group: Vec<usize>,
    gsize: Vec<usize>,
    nid: usize,
}

impl UnionFind {
    fn gp_find(&mut self, i: usize) -> usize {
        let gi = self.group[i];
        if i != gi {
            self.group[i] = self.gp_find(gi);
        }
        return self.group[i];
    }

    fn gp_size(&mut self, i: usize) -> usize {
        let gi = self.gp_find(i);
        self.gsize[gi]
    }

    fn gp_merge(&mut self, gi: usize, gj: usize) {
        self.group[gj] = gi;
        self.gsize[gi] += self.gsize[gj];
        self.gsize[gj] = 0;
    }

    fn add(&mut self, name: &str) -> usize {
        if !self.idmap.contains_key(name) {
            self.idmap.insert(name.to_string(), self.nid);
            self.group.push(self.nid);
            self.gsize.push(1);
            self.nid += 1;
        }
        self.idmap[name]
    }

    fn unite(&mut self, i: usize, j: usize) {
        let gi = self.gp_find(i);
        let gj = self.gp_find(j);
        if gi == gj {
            return;
        } 
        if self.gsize[gi] < self.gsize[gj] {
            self.gp_merge(gi, gj);
        } else  {
            self.gp_merge(gj, gi);
        }
    }


}
fn main() {

    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let n = iter.next().unwrap().unwrap();
    let n: usize = n.parse().unwrap();
    // println!("n = {}", n);
    for _ in 0..n {
        let mut uf = UnionFind {
            idmap: HashMap::new(),
            group: Vec::new(),
            gsize: Vec::new(),
            nid: 0,
        };
        let t = iter.next().unwrap().unwrap();
        let t: usize = t.parse().unwrap();
        // println!("t = {}", t);
        for _ in 0..t {
            let pair = iter.next().unwrap().unwrap();
            let pair = pair.split(' ').collect::<Vec<_>>();
            // println!("pair = {:?}", pair);
            let i = uf.add(pair[0]);
            let j = uf.add(pair[1]);
            uf.unite(i, j);
            println!("{}", uf.gp_size(i));
            // println!("{:?}", uf);
        }
    }
}
