use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let iter = stdin.lock().lines();

    for line in iter.map(|l| l.unwrap()) {
        let res = line.split('-')
                      .map(|name| name.chars().next().unwrap())
                      .collect::<String>();
        println!("{}", res);
    }
  
}
