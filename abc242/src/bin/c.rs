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
    }
    let mut dp = vec![vec![0; 11]; n];
    for x in 1..10 {
        dp[0][x] = 1;
    }
    for i in 1..n {
        for x in 1..10 {
            for dx in 0..3 {
                dp[i][x] = (dp[i][x] + dp[i - 1][x + dx - 1]) % M;
            }
        }
    }
    let mut result = 0;
    for x in 1..10 {
        result = (result + dp[n - 1][x]) % M;
    }
    println!("{}", result);
}
