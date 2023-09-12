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
    let lim = 111111111111111111;
    let mut nums = vec![lim];
    for d in -9..=9 {
        for mut s in 1..=9 {
            let mut x = s;
            nums.push(x);
            while x < lim && 0 <= s + d && s + d <= 9 {
                s += d;
                x *= 10;
                x += s;
                nums.push(x);
            }
        }
    }
    nums.sort();
    nums.dedup();
    println!("{}", nums[nums.lower_bound(&x)]);
}
