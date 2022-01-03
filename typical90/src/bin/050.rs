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
        l: usize,
    }
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 0..n {
        if i + l <= n {
            dp[i + l] = (dp[i + l] + dp[i]) % M;
        }
        dp[i + 1] = (dp[i + 1] + dp[i]) % M;
    }
    println!("{}", dp[n]);
}
