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
        k: usize,
    }
    let d = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut nums = vec![];
    for x in 1..1 << d.len() {
        let mut s = 0usize;
        for j in 0..d.len() {
            if x & 1 << j > 0 {
                s *= 10;
                s += d[j];
            }
        }
        nums.push(s);
    }
    nums.sort();
    println!("{}", nums[k]);
}
