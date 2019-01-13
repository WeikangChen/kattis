use std::io::{self, BufRead};
use std::collections::HashMap;
use std::ops::BitXor;

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
 
    loop {
        let line = iter.next().unwrap().unwrap();
        let cnt: usize = line.parse().unwrap();
        // println!("cnt={}", cnt);
        if cnt == 0 {
            break;
        }
        
        let mut hash_cnt = HashMap::new();
        let mut coll_cnt = vec![0; 256];
        for _ in 0..cnt {
            let line = iter.next().unwrap().unwrap();
            let bit = line.as_bytes().iter().fold(0, u8::bitxor) as usize;
            coll_cnt[bit] += 1;

            let cnt =  hash_cnt.entry(line).or_insert(0);
            *cnt  += 1;
            // println!("line={} bit={} coll_cnt={} hash_cnt={}", 
            //         line, bit, coll_cnt[bit], hash_cnt[&line]);
        }

        let mut coll = 0;
        for i in 0..256 {
            let cnt = coll_cnt[i];
            if cnt > 0 {
                coll += cnt * (cnt - 1) / 2;
            }
        }
        for cnt in hash_cnt.values() {
            coll -= cnt * (cnt - 1) / 2;
        }
        println!("{} {}", hash_cnt.len(), coll);
            
    }
}
