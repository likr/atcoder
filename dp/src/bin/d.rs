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
      w: usize,
      wv: [(usize, usize); n],
    }
    let mut dp = vec![0; w + 1];
    for i in 0..n {
        let (wi, vi) = wv[i];
        for j in (wi..=w).rev() {
            dp[j] = max(dp[j], dp[j - wi] + vi);
        }
    }
    println!("{}", dp[w]);
}
