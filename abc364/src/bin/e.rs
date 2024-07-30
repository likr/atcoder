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
        x: usize,
        y: usize,
        ab: [(usize, usize); n],
    }
    let mut dp = vec![vec![INF; x + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (ai, bi) = ab[i];
        for i in (0..n).rev() {
            for j in 0..=x {
                if j + ai <= x {
                    dp[i + 1][j + ai] = min(dp[i + 1][j + ai], dp[i][j] + bi);
                }
            }
        }
    }
    let mut ans = 0;
    for i in 1..=n {
        if (0..=x).any(|j| dp[i][j] <= y) {
            ans = i;
        }
    }
    if ans != n {
        ans += 1;
    }
    println!("{}", ans);
}
