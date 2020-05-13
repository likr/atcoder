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

fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize,
        th: [(usize, usize); n],
    }
    let m = x + y;
    let mut dp = vec![vec![0; m + 1]; n];
    for i in 0..n {
        let (ti, hi) = th[i];
        for j in (1..n).rev() {
            for k in (ti..=m).rev() {
                if dp[j - 1][k - ti] > 0 {
                    dp[j][k] = max(dp[j][k], dp[j - 1][k - ti] + hi);
                }
            }
        }
        for k in ti..=m {
            dp[0][k] = max(dp[0][k], hi);
        }
    }
    let mut result = 0;
    for j in 0..min(n, x) {
        // eprintln!("{:?}", dp[j]);
        result = max(result, dp[j][m]);
    }
    println!("{}", result);
}
