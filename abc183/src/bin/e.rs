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
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut dp = vec![vec![0; w + 1]; h + 1];
    let mut acc_v = vec![0; w + 1];
    let mut acc_x = vec![0; w + 1];
    dp[1][1] = 1usize;
    for i in 1..=h {
        let mut acc_h = 0;
        for j in 1..=w {
            if s[i - 1][j - 1] == '#' {
                acc_h = 0;
                continue;
            }
            dp[i][j] = (dp[i][j] + acc_h) % M;
            dp[i][j] = (dp[i][j] + acc_v[j]) % M;
            dp[i][j] = (dp[i][j] + acc_x[j - 1]) % M;
            acc_h = (acc_h + dp[i][j]) % M;
        }
        for j in 1..=w {
            if s[i - 1][j - 1] == '#' {
                acc_v[j] = 0;
            } else {
                acc_v[j] = (acc_v[j] + dp[i][j]) % M;
            }
        }
        for j in (1..=w).rev() {
            if s[i - 1][j - 1] == '#' {
                acc_x[j] = 0;
            } else {
                acc_x[j] = (acc_x[j - 1] + dp[i][j]) % M;
            }
        }
        // debug!(acc_h);
        // debug!(acc_v);
        // debug!(acc_x);
    }
    println!("{}", dp[h][w]);
}
