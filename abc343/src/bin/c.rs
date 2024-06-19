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
        n: usize,
    }
    let mut nums = vec![];
    for x in 1.. {
        if x * x * x > n {
            break;
        }
        let s = format!("{}", x * x * x);
        let t = s.chars().rev().collect::<String>();
        if s == t {
            nums.push(x * x * x);
        }
    }
    nums.reverse();
    println!("{}", nums[0]);
}
