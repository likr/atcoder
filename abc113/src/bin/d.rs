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
        h: usize,
        w: usize,
        k: usize,
    }
    let base = (0..w).collect::<Vec<_>>();
    let mut patterns = vec![];
    for x in 0..2usize.pow(w as u32 - 1) {
        let pattern = (0..w - 1)
            .map(|i| if x & (1 << i) > 0 { 1 } else { 0 })
            .collect::<Vec<_>>();
        if (1..pattern.len()).all(|i| pattern[i - 1] == 0 || pattern[i] == 0) {
            let mut y = base.clone();
            for (i, &p) in pattern.iter().enumerate() {
                if p == 1 {
                    y.swap(i, i + 1);
                }
            }
            patterns.push(y);
        }
    }
    // eprintln!("{:?}", patterns);
    let mut dp = vec![vec![0; w]; h + 1];
    dp[0][0] = 1;
    for i in 0..h {
        for pattern in &patterns {
            for j in 0..w {
                dp[i + 1][pattern[j]] = (dp[i + 1][pattern[j]] + dp[i][j]) % M;
            }
        }
    }
    println!("{}", dp[h][k - 1]);
}
