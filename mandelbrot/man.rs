use std::io::{self, BufRead};
use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Complex {
    re: f64,
    im: f64
}

impl Complex {
    fn sq2(self) -> Complex {
        Complex {
            re: self.re * self.re - self.im * self.im,
            im: 2. * self.re * self.im
        }
    }

    fn norm(self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        if self.im >= 0.0 {
            write!(f, "{}+{}i", self.re, self.im)
        } else {
            write!(f, "{}{}i", self.re, self.im)
        }
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

fn solve(c: Complex, r: i32) -> &'static str {
    // println!("c={} r={}", c, r);
    let mut z = Complex{re: 0.0, im: 0.0};
    let mut man = true;
    for _ in 0..r {
        z = z.sq2() + c;
        if z.norm() > 2.0 {
            // println!("z={}", z);
            man = false;
            break;
        }
    }

    if man {
        "IN"
    } else {
        "OUT"
    }
}

fn main() {
    let stdin = io::stdin();
    let iter = stdin.lock().lines();
    for (i, line) in iter.map(|l| l.unwrap()).enumerate() {
        let v: Vec<f64> = line.split_whitespace()
                              .map(|num| num.parse().unwrap())
                              .collect();
        // println!("{}", line);
        // println!("{:?}", v);
        let z = Complex{re:v[0], im:v[1]};
        println!("Case {}: {}", i + 1, solve(z, v[2] as i32));
    }
}
