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
        x: usize,
        mut a: [usize; n],
    }
    a.sort();
    let a = a.into_iter().enumerate().collect::<Vec<(usize, usize)>>();
    let result = a.upper_bound_by(|&(k, ak)| {
        if ak >= x {
            return Ordering::Greater;
        }
        let mut dp = vec![vec![false; x + 1]; n + 1];
        for i in 0..=n {
            dp[i][0] = true;
        }
        for i in 0..n {
            let ai = a[i].1;
            if i == k || ai > x {
                for j in 0..=x {
                    dp[i + 1][j] = dp[i][j];
                }
            } else {
                for j in 0..ai {
                    dp[i + 1][j] = dp[i][j];
                }
                for j in ai..=x {
                    dp[i + 1][j] = dp[i][j] || dp[i][j - ai];
                }
            }
        }
        if (x - ak..x).any(|j| dp[n][j]) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
    println!("{}", result);
}
