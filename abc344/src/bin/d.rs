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
        t: Chars,
        n: usize,
        s: [[Chars]; n],
    }
    let m = t.len();
    let mut dp = vec![vec![INF; m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..=m {
            dp[i + 1][j] = dp[i][j];
        }
        for j in 0..m {
            for k in 0..s[i].len() {
                if j + s[i][k].len() <= m && (0..s[i][k].len()).all(|l| s[i][k][l] == t[j + l]) {
                    dp[i + 1][j + s[i][k].len()] = min(dp[i + 1][j + s[i][k].len()], dp[i][j] + 1);
                }
            }
        }
    }
    if dp[n][m] == INF {
        println!("-1");
    } else {
        println!("{}", dp[n][m]);
    }
}
