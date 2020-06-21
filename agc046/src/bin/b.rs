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
const M: usize = 998244353;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let mut dp = vec![vec![0; d + 1]; c + 1];
    dp[a][b] = 1;
    for i in a + 1..=c {
        dp[i][b] = b * dp[i - 1][b] % M;
    }
    for j in b + 1..=d {
        dp[a][j] = a * dp[a][j - 1] % M;
    }
    for i in a + 1..=c {
        for j in b + 1..=d {
            dp[i][j] = ((dp[i - 1][j] * j % M) + (dp[i][j - 1] * i % M) + M
                - ((i - 1) * (j - 1) * dp[i - 1][j - 1]) % M)
                % M;
        }
    }
    println!("{}", dp[c][d]);
}
