use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        mut d: [usize; n],
    }
    d.sort();
    let mut dp = vec![vec![0; 4]; n];
    for i in 0..n {
        dp[i][0] = i + 1;
    }
    for i in 1..n {
        for j in 1..4 {
            dp[i][j] = dp[i - 1][j];
        }

        let k = d.upper_bound(&(d[i] / 2));
        // eprintln!("{} {}", i, k);
        if k > 0 {
            for j in 1..4 {
                dp[i][j] = (dp[i][j] + dp[k - 1][j - 1]) % M;
            }
        }
    }
    // for j in 0..4 {
    //     eprintln!("{:?}", (0..n).map(|i| dp[i][j]).collect::<Vec<_>>());
    // }
    println!("{}", dp[n - 1][3]);
}
