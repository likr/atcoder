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
        n: Chars,
        k: usize,
    }
    let n = n
        .into_iter()
        .map(|c| c as usize - '0' as usize)
        .rev()
        .collect::<Vec<_>>();
    let m = n.len();
    let mut dp = vec![vec![(0, 0); k + 1]; m + 1];
    dp[0][0] = (1, 0);
    for i in 0..m {
        for j in 0..=k {
            if n[i] == 0 {
                dp[i + 1][j].0 += dp[i][j].0;
            } else {
                dp[i + 1][j].1 += dp[i][j].0;
            }
            dp[i + 1][j].1 += dp[i][j].1;
        }
        for j in 0..k {
            if n[i] > 0 {
                dp[i + 1][j + 1].0 += dp[i][j].0;
                dp[i + 1][j + 1].1 += dp[i][j].0 * (n[i] - 1);
            }
            dp[i + 1][j + 1].1 += dp[i][j].1 * 9;
        }
    }
    println!("{}", dp[m][k].0 + dp[m][k].1);
}
