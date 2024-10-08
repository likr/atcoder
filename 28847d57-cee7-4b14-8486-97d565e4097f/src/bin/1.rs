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
        h: [i64; n],
    }
    let mut dp = vec![0; n];
    dp[1] = (h[0] - h[1]).abs();
    for i in 2..n {
        dp[i] = min(
            dp[i - 2] + (h[i - 2] - h[i]).abs(),
            dp[i - 1] + (h[i - 1] - h[i]).abs(),
        );
    }
    println!("{}", dp[n - 1]);
}
