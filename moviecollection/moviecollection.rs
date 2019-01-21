use std::io::{self, BufRead};

fn most_right_bit(i: usize) -> usize {
    assert!(i > 0);
    i & !(i - 1)
}

#[derive(Debug)]
struct FenwickTree {
    tree: Vec<i32>,
    maxi: usize,
}

impl FenwickTree {

    fn update(&mut self, mut i: usize, delta: i32) {
        while i <= self.maxi {
            self.tree[i] += delta;
            i += most_right_bit(i);
        }
    }

    fn sum(&mut self, mut i: usize) -> i32 {
        assert!(i <= self.maxi);
        let mut res = 0;
        while i > 0 {
            res += self.tree[i];
            i -= most_right_bit(i);
        }
        res
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let n = iter.next().unwrap().unwrap();
    let n: usize = n.parse().unwrap();
    // println!("n = {}", n);

    for _ in 0..n {
        let line = iter.next().unwrap().unwrap();
        let pair = line.split(' ')
                       .map(|n| n.parse().unwrap())
                       .collect::<Vec<usize>>();
        // println!("pair = {:?}", pair);
        let (m, r) = (pair[0], pair[1]);

        let mut ftree = FenwickTree {
            tree: vec![0; m + r + 1],
            maxi: m + r,
        };
        let mut pos = vec![0; m + 1];
        for i in 1..=m {
            pos[i] = m - i + 1;
            ftree.update(pos[i], 1);
        }

        let line = iter.next().unwrap().unwrap();
        let rs = line.split(' ')
                     .map(|n| n.parse().unwrap())
                     .collect::<Vec<usize>>();
        // println!("rs = {:?}", rs);
        let mut pos_n = m + 1;
        let mut ans = Vec::new();
        for ri in rs {
            let pos_i = pos[ri];
            let sum_i = ftree.sum(pos_i);
            // println!("{}: {} -> {}; ans={}", 
            //   ri, pos_i, pos_n, m as i32 - sum_i);
            ans.push((m as i32 - sum_i).to_string());
            ftree.update(pos_i, -1);
            ftree.update(pos_n, 1);
            pos[ri] = pos_n;
            pos_n += 1;
        }
        println!("{}", ans.join(" "));
    }

}
