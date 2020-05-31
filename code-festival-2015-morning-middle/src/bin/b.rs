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

fn lcs(a: &Vec<char>, b: &Vec<char>) -> usize {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            dp[i + 1][j + 1] = if a[i] == b[j] {
                dp[i][j] + 1
            } else {
                max(dp[i][j + 1], dp[i + 1][j])
            };
        }
    }
    dp[n][m]
}

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut result = INF;
    for k in 0..n {
        let left = (0..k).map(|i| s[i]).collect::<Vec<_>>();
        let right = (k..n).map(|i| s[i]).collect::<Vec<_>>();
        result = min(result, n - lcs(&left, &right) * 2);
    }
    println!("{}", result);
}
