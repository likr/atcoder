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
        k: usize,
    }

    let mut factors = vec![];
    for d in 1.. {
        if d * d > n {
            break;
        }
        factors.push(d);
        factors.push(n / d);
    }
    factors.sort();
    factors.dedup();
    let m = factors.len();

    let mut dp = vec![vec![0; m]; k];
    dp[0][0] = 1;
    for f in 1..m {
        dp[0][f] = factors[f] - factors[f - 1];
    }
    for i in 1..k {
        for j in 0..m {
            dp[i][j] = dp[i - 1][m - j - 1];
        }
        for j in (1..m).rev() {
            dp[i][j - 1] = (dp[i][j - 1] + dp[i][j]) % M;
        }
        for j in 1..m {
            dp[i][j] = dp[i][j] * (factors[j] - factors[j - 1]) % M
        }
    }

    let mut result = 0usize;
    for j in 0..m {
        result = (result + dp[k - 1][j]) % M;
    }
    println!("{}", result);
}
