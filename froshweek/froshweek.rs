use std::io::{self, BufRead};

fn count_merge(nums: &mut Vec<i32>, l: usize, m: usize, r: usize) -> usize {
    let l_len = m - l;
    let r_len = r - m + 1; 
    let l_nums = nums[l..m].iter().cloned().collect::<Vec<_>>();
    let r_nums = nums[m..r+1].iter().cloned().collect::<Vec<_>>();
    
    let mut cnt = 0;
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < l_len && j < r_len {
        if l_nums[i] < r_nums[j] {
            nums[k] = l_nums[i];
            k += 1;
            i += 1;
        } else {
            nums[k] = r_nums[j];
            k += 1;
            j += 1;
            cnt += l_len - i;
        }
    }

    while i < l_len {
        nums[k] = l_nums[i];
        k += 1;
        i += 1;
    }
    while j < r_len {
        nums[k] = r_nums[j];
        k += 1;
        j += 1;
    }
    
    cnt
}

fn count_inv(nums: &mut Vec<i32>, l: usize, r: usize) -> usize {
    // println!("{:?} {} {}", nums, l, r);
    let mut cnt = 0;
    if l < r {
        let m = l + (r  - l) / 2;
        cnt += count_inv(nums, l, m);
        cnt += count_inv(nums, m + 1, r);
        cnt += count_merge(nums, l, m + 1, r);
    }
    cnt
}

fn count_swap(nums: &mut Vec<i32>, n: usize) -> usize {
    count_inv(nums, 0, n - 1)
}

/*
fn merge(nums: &mut Vec<i32>, 
         tmp: &mut Vec<i32>,
         l: usize, m: usize, r: usize) -> usize
{
    let mut cnt = 0;
    let mut i = l;
    let mut j = m;
    let mut k = l;
    while i < m && j <= r {
        if nums[i] < nums[j] {
            tmp[k] = nums[i];
            k += 1;
            i += 1;
        } else {
            tmp[k] = nums[j];
            k += 1;
            j += 1;
            cnt += m - i;
        }
    }

    while i < m {
        tmp[k] = nums[i];
        k += 1;
        i += 1;
    }
    while j <= r { 
        tmp[k] = nums[j];
        k += 1;
        j += 1;
    }
    for t in l..=r {
        nums[t] = tmp[t]
    }

    cnt
}

fn merge_sort(nums: &mut Vec<i32>, 
              tmp: &mut Vec<i32>, 
              l: usize, r: usize) -> usize 
{
    let mut cnt = 0;
    if l < r {
        let m = l + (r  - l) / 2;
        cnt += merge_sort(nums, tmp, l, m);
        cnt += merge_sort(nums, tmp, m + 1, r);
        cnt += merge(nums, tmp, l, m + 1, r);
    }
    cnt
}

fn count_swap(nums: &mut Vec<i32>, n: usize) -> usize {
    let mut tmp = vec!(0; n);
    merge_sort(nums, &mut tmp, 0, n - 1)
}
*/

#[allow(dead_code)]
fn bubble_sort(nums: &mut Vec<i32>, mut n: usize) -> usize {
    let mut cnt: usize = 0;
    loop {
        let mut swapped = false;
        for i in 1..n {
            if nums[i - 1] > nums[i] {
                nums.swap(i - 1, i);
                // println!("swap: {} {}", i-1, i);
                swapped = true;
                cnt += 1;
            }
        }
        if !swapped {
            break;
        }
        n -= 1;
    }
    cnt
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let line = iter.next().unwrap().unwrap();
    let n: usize = line.parse().unwrap();
    let mut nums = iter.map(|l| l.unwrap().parse().unwrap()).collect::<Vec<i32>>();
    // println!("n={} nums={:?}", n, nums);
    // println!("{}", bubble_sort(&mut nums, n));
    println!("{}", count_swap(&mut nums, n));
}
