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
        txa: [(usize, usize, usize); n],
    }
    let m = 100000;
    let mut dp = vec![vec![0; 5]; m + 1];
    let mut i = 0;
    for t in 1..=m {
        for j in 0..5 {
            dp[t][j] = dp[t - 1][j];
        }
        for j in 1..5 {
            dp[t][j] = max(dp[t][j], dp[t - 1][j - 1]);
        }
        for j in 0..4 {
            dp[t][j] = max(dp[t][j], dp[t - 1][j + 1]);
        }
        if i < n && txa[i].0 == t {
            if txa[i].1 <= t {
                dp[t][txa[i].1] += txa[i].2;
            }
            i += 1;
        }
    }
    println!("{}", (0..5).map(|j| dp[m][j]).max().unwrap());
}
