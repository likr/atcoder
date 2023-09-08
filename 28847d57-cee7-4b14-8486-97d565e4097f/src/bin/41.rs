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
        w: usize,
        wv: [(usize, usize); n],
    }
    let m = 100001;
    let mut dp = vec![vec![INF; m]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (wi, vi) = wv[i];
        for j in 0..m {
            if j < vi {
                dp[i + 1][j] = dp[i][j];
            } else {
                dp[i + 1][j] = min(dp[i][j], dp[i][j - vi] + wi);
            }
        }
    }
    let mut ans = 0;
    for j in 0..m {
        if dp[n][j] <= w {
            ans = j;
        }
    }
    println!("{}", ans);
}
