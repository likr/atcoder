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
        t: [usize; n],
    }
    let mut s = 0;
    for i in 0..n {
        s += t[i];
    }
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..s {
            if dp[i][j] {
                dp[i + 1][j] = true;
                if j + t[i] <= s {
                    dp[i + 1][j + t[i]] = true;
                }
            }
        }
    }
    let mut result = INF;
    for j in 0..=s {
        if dp[n][j] {
            result = min(result, max(j, s - j));
        }
    }
    println!("{}", result);
}
