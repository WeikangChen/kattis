use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let iter = stdin.lock().lines();
    for line in iter.map(|l| l.unwrap()) {
        let t: i32 = line.parse().unwrap();
        for i in 1..=t {
            println!("{} Abracadabra", i);
        }
    }
  
}
