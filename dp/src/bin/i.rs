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
      p: [f64; n],
    }
    let mut dp = vec![0.; n + 1];
    dp[0] = 1.;
    for i in 0..n {
        let pi = p[i];
        for j in (1..=i + 1).rev() {
            dp[j] = dp[j - 1] * pi + dp[j] * (1. - pi);
        }
        dp[0] *= 1. - pi;
    }
    // eprintln!("{:?}", dp);
    let mut s = 0.;
    for i in (n + 1) / 2..=n {
        s += dp[i];
    }
    println!("{}", s);
}
