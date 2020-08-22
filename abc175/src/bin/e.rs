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
        r: usize,
        c: usize,
        k: usize,
        rcv: [(usize, usize, usize); k],
    }
    let mut v = vec![vec![0; c + 1]; r + 1];
    for &(ri, ci, vi) in &rcv {
        v[ri][ci] = vi;
    }
    let mut dp = vec![vec![vec![0; 4]; c + 1]; r + 1];
    for i in 1..=r {
        for j in 1..=c {
            dp[i][j][0] = *dp[i - 1][j].iter().max().unwrap();
        }
        for j in 1..=c {
            dp[i][j][1] = dp[i][j - 1][1];
            if v[i][j] > 0 {
                dp[i][j][1] = max(dp[i][j][1], dp[i][j - 1][0] + v[i][j]);
                dp[i][j][1] = max(dp[i][j][1], dp[i - 1][j].iter().max().unwrap() + v[i][j]);
            }
        }
        for j in 1..=c {
            for k in 2..=3 {
                dp[i][j][k] = dp[i][j - 1][k];
                if v[i][j] > 0 {
                    dp[i][j][k] = max(dp[i][j][k], dp[i][j - 1][k - 1] + v[i][j]);
                }
            }
        }
    }
    // for k in 0..4 {
    //     for i in 0..=r {
    //         for j in 0..=c {
    //             eprint!("{:?} ", dp[i][j][k]);
    //         }
    //         eprintln!("");
    //     }
    //     eprintln!("");
    // }
    println!("{}", dp[r][c].iter().max().unwrap());
}
