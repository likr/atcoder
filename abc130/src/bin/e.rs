use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [usize; n],
        t: [usize; m],
    }
    let mut dp = vec![vec![1; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if s[i - 1] == t[j - 1] {
                (dp[i - 1][j] + dp[i][j - 1]) % M
            } else {
                (dp[i - 1][j] + dp[i][j - 1] - dp[i - 1][j - 1] + M) % M
            };
        }
    }
    println!("{}", dp[n][m]);
}
