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
      a: [Chars; h],
    }
    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = 1;
    for i in 1..h {
        if a[i][0] == '.' {
            dp[i][0] = dp[i - 1][0];
        }
    }
    for j in 1..w {
        if a[0][j] == '.' {
            dp[0][j] = dp[0][j - 1];
        }
    }
    for i in 1..h {
        for j in 1..w {
            if a[i][j] != '#' {
                dp[i][j] = (dp[i - 1][j] + dp[i][j - 1]) % M;
            }
        }
    }
    println!("{}", dp[h - 1][w - 1]);
}
