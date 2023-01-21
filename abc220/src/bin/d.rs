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
        a: [usize; n],
    }
    let mut dp = vec![vec![0; 10]; n];
    dp[1][(a[0] + a[1]) % 10] += 1;
    dp[1][(a[0] * a[1]) % 10] += 1;
    for i in 2..n {
        for j in 0..10 {
            dp[i][(j + a[i]) % 10] = (dp[i][(j + a[i]) % 10] + dp[i - 1][j]) % M;
            dp[i][(j * a[i]) % 10] = (dp[i][(j * a[i]) % 10] + dp[i - 1][j]) % M;
        }
    }
    for k in 0..10 {
        println!("{}", dp[n - 1][k]);
    }
}
