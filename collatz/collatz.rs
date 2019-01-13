use std::io::{self, BufRead};
use std::collections::HashMap;

fn collatz(x: u64) -> u64 {
    if x % 2 == 0 {
        x / 2
    } else {
        x * 3 + 1
    }
}

fn solve(a: u64, b: u64) {
    let mut dict = HashMap::new();

    let mut an = 0;
    let mut a1 = a;
    dict.insert(a1, an);
    while a1!= 1 {
        a1 = collatz(a1);
        an += 1;
        dict.insert(a1, an);
    }

    let mut bn = 0;
    let mut b1 = b;
    loop {
        match dict.get(&b1) {
            Some(v) => {
                an = *v;
                break;
            },
            None => {
                b1 = collatz(b1);
                bn += 1;
            },
        }
    }

    println!("{} needs {} steps, {} needs {} steps, they meet at {}", a, an, b, bn, b1);
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    
    loop {
        let line1 = iter.next().unwrap().unwrap();
        // println!("{}", line1);
        let v: Vec<u64> = line1.split_whitespace().map(|num| num.parse().unwrap()).collect();
        if v[0] == 0 {
            break;
        }
        solve(v[0], v[1]);
    }
}

#[test]
fn test() {
    for i in 1..=1000000 {
        let mut i = i;
        while i != 1 {
             i = collatz(i);
        }
    }

}
