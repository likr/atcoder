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
      h: [i64; n],
    }
    let mut dp = vec![0; n];
    dp[1] = (h[0] - h[1]).abs();
    for i in 2..n {
        dp[i] = min(
            dp[i - 2] + (h[i] - h[i - 2]).abs(),
            dp[i - 1] + (h[i] - h[i - 1]).abs(),
        );
    }
    println!("{}", dp[n - 1]);
}
