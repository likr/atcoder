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
        w: usize,
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }
    let mut dp = vec![vec![vec![0usize; w + 1]; k + 1]; n + 1];
    for i in 1..=n {
        let (ai, bi) = ab[i - 1];
        for j in 1..=k {
            for l in 0..min(ai, w + 1) {
                dp[i][j][l] = dp[i - 1][j][l];
            }
            for l in ai..=w {
                dp[i][j][l] = max(dp[i - 1][j][l], bi + dp[i - 1][j - 1][l - ai]);
            }
        }
    }
    let mut result = 0usize;
    for j in 0..=k {
        for l in 0..=w {
            // eprint!("{} ", dp[n][j][l]);
            result = max(result, dp[n][j][l]);
        }
        // eprintln!("");
    }
    println!("{}", result);
}
