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
    let mut dp = vec![0.; n];
    dp[0] = 3.5;
    for i in 1..n {
        let mut s = 0.;
        for j in 1..=6 {
            if j as f64 > dp[i - 1] {
                s += j as f64;
            } else {
                s += dp[i - 1];
            }
        }
        dp[i] = s / 6.;
    }
    println!("{}", dp[n - 1]);
}
