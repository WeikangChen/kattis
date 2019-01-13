use std::io::{self, BufRead};
// use std::ops::{Add, Sub};
use std::collections::HashMap;

struct Words {
    var2num: HashMap<String, i32>,
    num2var: HashMap<i32, String>,
}

impl Words {
    fn def(&mut self, strs: &[&str]) {
        let var = String::from(strs[0]);
        let num: i32 = strs[1].parse().unwrap();
        // println!("def {} {}", var, num);
        match self.var2num.remove(&var) {
            Some(old) => {
                self.num2var.remove(&old);
            },
            None => (),
        }
        self.var2num.insert(var.clone(), num);
        self.num2var.insert(num, var.clone());
    }
  
    fn clear(&mut self) {
        self.var2num.clear();
        self.num2var.clear();
    }
  
    fn calc(&self, strs: &[&str]) {
        // println!("calc {:?} ", strs);
        let mut res = 0;
        let mut ans = "unknown";

        let add = |a:i32, b: i32| a + b;
        let sub = |a:i32, b: i32| a - b;
        let mut opt : &Fn(i32, i32) -> i32 = &add;

        for &str in strs {
            match str {
                "+" => opt = &add, 
                "-" => opt = &sub, 
                "=" => {
                    ans = match self.num2var.get(&res) {
                        Some(var) => var,
                        None => "unknown"
                    };
                    break;
                },
                var => {
                    match self.var2num.get(var) {
                        Some(&num) => res = opt(res, num),
                        None => break,
                    }
                    // println!("var={} res={}", var, res);
                },
            }
        }
        println!("{} {}", strs.join(" "), ans);
    }      
}


fn main() {
    let stdin = io::stdin();
    let iter = stdin.lock().lines();
    let mut words = Words {
        var2num: HashMap::new(),
        num2var: HashMap::new()
    };
    for line in iter.map(|l| l.unwrap()) {
        let strs: Vec<_> = line.split(' ').collect();
        match strs[0] {
            "def" => words.def(&strs[1..]),
            "calc" => words.calc(&strs[1..]),
            "clear" => words.clear(),
            _ =>  (),
        }
    }
}
