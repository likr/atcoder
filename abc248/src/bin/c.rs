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
        m: usize,
        k: usize,
    }
    let mut dp = vec![vec![0; k + 1]; n + 1];
    for j in 1..=m {
        dp[1][j] = 1;
    }
    for i in 2..=n {
        for j in 1..=k {
            for j1 in 1..j {
                let j2 = j - j1;
                dp[i][j] = (dp[i][j] + (dp[i - 1][j1] * dp[1][j2] % M)) % M;
            }
        }
    }
    let mut result = 0;
    for j in 1..=k {
        result = (result + dp[n][j]) % M;
    }
    println!("{}", result);
}
