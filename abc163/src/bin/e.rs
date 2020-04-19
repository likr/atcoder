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
        a: [usize; n],
    }

    let mut a = a
        .iter()
        .enumerate()
        .map(|(i, ai)| (ai, i))
        .collect::<Vec<_>>();
    a.sort();
    a.reverse();

    let mut dp = vec![vec![0; n + 1]; n + 1];
    for k in 0..n {
        let (ai, i) = a[k];
        for x in 0..=k {
            let y = k - x;
            let left = max(i, x) - min(i, x);
            dp[x + 1][y] = max(dp[x + 1][y], dp[x][y] + ai * left);
            let right = max(i, n - y - 1) - min(i, n - y - 1);
            dp[x][y + 1] = max(dp[x][y + 1], dp[x][y] + ai * right);
        }
    }

    let mut result = 0;
    for x in 0..=n {
        let y = n - x;
        result = max(result, dp[x][y]);
    }
    println!("{}", result);
}
