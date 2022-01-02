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
        s: Chars,
    }
    let target = "atcoder".chars().collect::<Vec<_>>();
    let mut dp = vec![vec![0; n + 1]; target.len() + 1];
    for j in 0..=n {
        dp[0][j] = 1;
    }
    for i in 1..=target.len() {
        for j in (0..n).rev() {
            dp[i][j] = dp[i][j + 1];
            if target[target.len() - i] == s[j] {
                dp[i][j] += dp[i - 1][j];
            }
            dp[i][j] = dp[i][j] % M;
        }
    }
    println!("{}", dp[target.len()][0]);
}
