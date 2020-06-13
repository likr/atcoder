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
        s: Chars,
        c: [usize; n],
        d: [usize; n],
    }
    let mut dp = vec![vec![INF; n + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        let ci = c[i - 1];
        let di = d[i - 1];
        let e = s[i - 1];
        for j in 0..=n {
            dp[i][j] = min(dp[i][j], dp[i - 1][j] + di);
        }
        for j in 1..=n {
            if e == '(' {
                dp[i][j] = min(dp[i][j], dp[i - 1][j - 1]);
            } else {
                dp[i][j] = min(dp[i][j], dp[i - 1][j - 1] + ci);
            }
        }
        for j in 0..n {
            if e == '(' {
                dp[i][j] = min(dp[i][j], dp[i - 1][j + 1] + ci);
            } else {
                dp[i][j] = min(dp[i][j], dp[i - 1][j + 1]);
            }
        }
    }
    // for i in 1..=n {
    //     eprintln!("{:?}", dp[i]);
    // }
    println!("{}", dp[n][0]);
}
