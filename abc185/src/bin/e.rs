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
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        dp[i + 1][0] = i + 1;
    }
    for j in 0..m {
        dp[0][j + 1] = j + 1;
    }
    for i in 0..n {
        for j in 0..m {
            dp[i + 1][j + 1] = min(dp[i][j + 1], dp[i + 1][j]) + 1;
            if a[i] == b[j] {
                dp[i + 1][j + 1] = min(dp[i + 1][j + 1], dp[i][j]);
            } else {
                dp[i + 1][j + 1] = min(dp[i + 1][j + 1], dp[i][j] + 1);
            };
        }
    }
    println!("{}", dp[n][m]);
}
