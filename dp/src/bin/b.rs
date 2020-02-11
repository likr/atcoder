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
const INF: i64 = std::i64::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
      n: usize,
      k: usize,
      h: [i64; n],
    }
    let mut dp = vec![INF; n];
    dp[0] = 0;
    for i in 0..n {
        for j in i + 1..min(i + k + 1, n) {
            dp[j] = min(dp[j], dp[i] + (h[i] - h[j]).abs());
        }
    }
    println!("{}", dp[n - 1]);
}
