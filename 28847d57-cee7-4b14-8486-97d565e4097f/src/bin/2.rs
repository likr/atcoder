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
        k: usize,
        h: [i64; n],
    }
    let mut dp = vec![INF as i64; n];
    dp[0] = 0;
    for i in 1..n {
        dp[i] = (1..=k)
            .filter(|&j| j <= i)
            .map(|j| dp[i - j] + (h[i - j] - h[i]).abs())
            .min()
            .unwrap();
    }
    println!("{}", dp[n - 1]);
}
