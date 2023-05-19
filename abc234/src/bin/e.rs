use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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
        x: i64,
    }
    let max = 111111111111111111i64;

    let mut nums = vec![max];
    for i in 1..=9 {
        let mut v = i;
        while v < max {
            nums.push(v);
            v = v * 10 + i;
        }
    }
    for i in 1..=9 {
        for j in 1..=9 {
            let mut w = i + j;
            if w > 9 {
                break;
            }
            let mut v = i * 10 + w;
            while v < max && w <= 9 {
                nums.push(v);
                w += j;
                v = v * 10 + w;
            }
        }
    }
    for i in 1..=9 {
        for j in 1..=9 {
            let mut w = i - j;
            if w < 0 {
                break;
            }
            let mut v = i * 10 + w;
            while v < max && w >= 0 {
                nums.push(v);
                w -= j;
                v = v * 10 + w;
            }
        }
    }
    nums.sort();
    println!("{}", nums[nums.lower_bound(&x)]);
}
