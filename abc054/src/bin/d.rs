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
        n: usize,
        ma: usize,
        mb: usize,
        abc: [(usize, usize, usize); n],
    }
    let s_a = abc.iter().map(|&(ai, _, _)| ai).sum::<usize>();
    let s_b = abc.iter().map(|&(_, bi, _)| bi).sum::<usize>();
    let mut dp = vec![vec![vec![INF; s_b + 1]; s_a + 1]; n + 1];
    dp[0][0][0] = 0;
    for i in 1..=n {
        let (ai, bi, ci) = abc[i - 1];
        for x in 0..=s_a {
            for y in 0..=s_b {
                if dp[i - 1][x][y] < INF {
                    dp[i][x][y] = min(dp[i - 1][x][y], dp[i][x][y]);
                    if x + ai <= s_a && y + bi <= s_b {
                        dp[i][x + ai][y + bi] = dp[i - 1][x][y] + ci;
                    }
                }
            }
        }
    }
    let mut k = 1;
    let mut result = INF;
    while ma * k <= s_a && mb * k <= s_b {
        result = min(result, dp[n][ma * k][mb * k]);
        k += 1;
    }
    if result == INF {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
