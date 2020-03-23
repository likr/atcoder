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
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut dp = vec![vec![(0, false); w]; h];
    if s[0][0] == '.' {
        dp[0][0] = (0, true);
    } else {
        dp[0][0] = (1, false);
    }
    for k in 1..h + w - 1 {
        for i in 0..=k {
            let j = k - i;
            if i >= h || j >= w {
                continue;
            }
            // eprintln!("{} {} {}", k, i, j);
            let (d0, b0) = if i == 0 {
                dp[i][j - 1]
            } else if j == 0 {
                dp[i - 1][j]
            } else {
                min(dp[i][j - 1], dp[i - 1][j])
            };
            dp[i][j] = if s[i][j] == '.' {
                (d0, true)
            } else if b0 {
                (d0 + 1, false)
            } else {
                (d0, false)
            };
        }
    }
    println!("{}", dp[h - 1][w - 1].0);
}
