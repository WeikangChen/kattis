use std::io::{self, BufRead};
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
struct Time {
    hh: u32,
    mm: u32,
    ss: u32
}

impl FromStr for Time {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<_> = s.split(':').collect();
        let hh = nums[0].parse::<u32>()?;
        let mm = nums[1].parse::<u32>()?;
        let ss = nums[2].parse::<u32>()?;
        Ok(Time{hh, mm, ss})
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{hh:02}:{mm:02}:{ss:02}", 
               hh=self.hh, mm=self.mm, ss=self.ss)
    }
}

impl Add for Time {
    type Output = Time;
    fn add(self, rhs: Time) -> Self::Output {
        Time {
            hh: self.hh + rhs.hh,
            mm: self.mm + rhs.mm,
            ss: self.ss + rhs.ss
        }
    }
}

impl Sub for Time {
    type Output = Time;
    fn sub(self, rhs: Time) -> Self::Output {
        static DAY_SS: u32 = 24 * 3600;

        let ss_tot_lhs = self.hh * 3600 + self.mm * 60 + self.ss; 
        let ss_tot_rhs = rhs.hh * 3600 + rhs.mm * 60 + rhs.ss; 
        
        let diff = if ss_tot_rhs >= ss_tot_lhs {
            ss_tot_lhs - ss_tot_rhs + DAY_SS
        } else {
            ss_tot_lhs - ss_tot_rhs
        };
        let hh = diff / 3600;
        let mm = (diff - hh * 3600) / 60;
        let ss = diff % 60;
        Time {
            hh: hh,
            mm: mm,
            ss: ss
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let line1 = iter.next().unwrap().unwrap();
    let line2 = iter.next().unwrap().unwrap();
    let t1: Time = line1.parse().unwrap();
    let t2: Time = line2.parse().unwrap();

    println!("{}", t2 - t1);
}
