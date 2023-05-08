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
const M: usize = 998244353;

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
        ab: [(usize, usize); n],
    }
    let mut dp = vec![(0usize, 0usize); n];
    dp[0] = (1, 1);
    for i in 1..n {
        if ab[i - 1].0 != ab[i].0 {
            dp[i].0 = (dp[i].0 + dp[i - 1].0) % M;
        }
        if ab[i - 1].1 != ab[i].0 {
            dp[i].0 = (dp[i].0 + dp[i - 1].1) % M;
        }
        if ab[i - 1].0 != ab[i].1 {
            dp[i].1 = (dp[i].1 + dp[i - 1].0) % M;
        }
        if ab[i - 1].1 != ab[i].1 {
            dp[i].1 = (dp[i].1 + dp[i - 1].1) % M;
        }
    }
    println!("{}", (dp[n - 1].0 + dp[n - 1].1) % M);
}
