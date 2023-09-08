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
    let mut dp = vec![vec![0; w + 1]; n + 1];
    for i in 0..n {
        let (wi, vi) = wv[i];
        for j in 0..=w {
            if wi <= j {
                dp[i + 1][j] = max(dp[i][j], dp[i][j - wi] + vi);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }
    println!("{}", dp[n].iter().max().unwrap());
}
