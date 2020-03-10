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
        m: usize,
        k: usize,
        a: [isize; n],
    }
    let mut dp = vec![vec![-1isize; n + 1]; m + 1];
    dp[0][0] = 0;
    for i in 1..=m {
        for j in 1..=n {
            for l in 1..=k {
                if j >= l && dp[i - 1][j - l] >= 0 {
                    dp[i][j] = max(dp[i][j], a[j - 1] + dp[i - 1][j - l]);
                }
            }
        }
    }
    // for i in 1..=m {
    //     eprintln!("{:?}", dp[i]);
    // }
    let mut result = 0;
    for j in n - k + 1..=n {
        result = max(result, dp[m][j]);
    }
    if result == 0 {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
