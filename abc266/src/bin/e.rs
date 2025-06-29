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
    let mut dp = vec![vec![0f64; 6]; n];
    for j in 0..6 {
        dp[0][j] = (j + 1) as f64;
    }
    for i in 1..n {
        let mut s = 0.;
        for j in 0..6 {
            s += dp[i - 1][j];
        }
        s /= 6.;
        for j in 0..6 {
            dp[i][j] = if s > (j + 1) as f64 {
                s
            } else {
                (j + 1) as f64
            };
        }
    }
    let mut s = 0.;
    for j in 0..6 {
        s += dp[n - 1][j];
    }
    s /= 6.;
    println!("{}", s);
}
