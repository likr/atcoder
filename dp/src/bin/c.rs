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
      abc: [(usize, usize, usize); n],
    }
    let mut dp = vec![vec![0; 3]; n];
    dp[0][0] = abc[0].0;
    dp[0][1] = abc[0].1;
    dp[0][2] = abc[0].2;
    for i in 1..n {
        let (ai, bi, ci) = abc[i];
        dp[i][0] = max(dp[i - 1][1], dp[i - 1][2]) + ai;
        dp[i][1] = max(dp[i - 1][0], dp[i - 1][2]) + bi;
        dp[i][2] = max(dp[i - 1][0], dp[i - 1][1]) + ci;
    }
    println!("{}", max(max(dp[n - 1][0], dp[n - 1][1]), dp[n - 1][2]));
}
