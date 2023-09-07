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
        abc: [[usize; 3]; n],
    }
    let mut dp = vec![vec![0; 3]; n + 1];
    for i in 0..n {
        for j in 0..3 {
            dp[i + 1][j] = abc[i][j] + max(dp[i][(j + 1) % 3], dp[i][(j + 2) % 3]);
        }
    }
    println!("{}", dp[n].iter().max().unwrap());
}
