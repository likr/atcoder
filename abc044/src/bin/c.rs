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
        a: usize,
        x: [usize; n],
    }
    let s = 2500;
    let mut dp = vec![vec![vec![0usize; s + 1]; n + 1]; n + 1];
    for i in 1..=n {
        let xi = x[i - 1];
        dp[i][1][xi] += 1;
        for j in 2..=n {
            for k in 1..=s {
                if dp[i - 1][j - 1][k] > 0 {
                    dp[i][j - 1][k] += dp[i - 1][j - 1][k];
                    dp[i][j][k + xi] += dp[i - 1][j - 1][k];
                }
            }
        }
    }
    // for i in 1..=n {
    //     eprintln!("{}", i);
    //     for j in 1..=n {
    //         eprintln!("{:?}", dp[i][j]);
    //     }
    // }

    let mut count = 0;
    for i in 1..=n {
        count += dp[n][i][a * i];
    }
    println!("{}", count);
}
