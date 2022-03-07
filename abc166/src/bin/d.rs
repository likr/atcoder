use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        x: usize,
    }
    let mut nums = vec![];
    for a in 0usize.. {
        if a.pow(5) - (a - 1).pow(5) > x {
            break;
        }
        nums.push((a.pow(5), a));
    }
    for i in 0..nums.len() {
        for j in 0..i {
            if nums[i].0 - nums[j].0 == x {
                println!("{} {}", nums[i].1, nums[j].1);
                return;
            }
            if nums[i].0 + nums[j].0 == x {
                println!("{} -{}", nums[i].1, nums[j].1);
                return;
            }
        }
    }
}
