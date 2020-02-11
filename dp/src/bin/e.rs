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
    let mut s = 0;
    for &(_, vi) in &wv {
        s += vi;
    }
    let mut dp = vec![INF; s + 1];
    dp[0] = 0;
    for i in 0..n {
        let (wi, vi) = wv[i];
        for j in (0..=s).rev() {
            if dp[j] != INF {
                dp[j + vi] = min(dp[j + vi], dp[j] + wi);
            }
        }
    }
    let mut max = 0;
    for i in 0..=s {
        if dp[i] <= w {
            max = i;
        }
    }
    println!("{}", max);
}
