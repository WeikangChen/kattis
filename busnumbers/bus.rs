use std::io::{self, BufRead};


fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let _ = iter.next();
    let line = iter.next().unwrap().unwrap();
    let mut vs: Vec<i32> = line.split_whitespace()
                          .map(|num| num.parse().unwrap())
                          .collect();
    vs.sort();
/*
    let vs: Vec<_> = vs.iter()
                       .map(|v| v.to_string())
                       .collect();
*/
    // println!("{}", vs.join(" "));

    let mut iter = vs.into_iter();
    let mut prev = iter.next().unwrap();
    let mut len = 1;
    let mut res = prev.to_string();
    loop {
        match iter.next() {
            Some(v) => {
                if v == prev + 1 {
                    len += 1
                } else {
                    if len == 2 {
                        res.push_str(" ");
                        res.push_str(&prev.to_string());
                    } else if len > 2 {
                        res.push_str("-");
                        res.push_str(&prev.to_string());
                    }
                    len = 1;
                    res.push_str(" ");
                    res.push_str(&v.to_string());
                }
                prev = v;
            },
            _ => {
                if len == 2 {
                    res.push_str(" ");
                    res.push_str(&prev.to_string());
                } else if len > 2 {
                    res.push_str("-");
                    res.push_str(&prev.to_string());
                }
                break;
            },
        }
    }
    println!("{}", res);
}
