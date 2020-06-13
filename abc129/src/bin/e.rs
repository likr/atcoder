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
        mut l: Chars,
    }
    l.reverse();
    let l = l
        .into_iter()
        .map(|c| c as usize - '0' as usize)
        .collect::<Vec<_>>();
    let n = l.len();
    let mut dp = vec![(0, 0); n];
    if l[0] == 0 {
        dp[0] = (3, 1);
    } else {
        dp[0] = (3, 3);
    }
    for i in 1..n {
        dp[i].0 = (dp[i - 1].0 * 3) % M;
        if l[i] == 0 {
            dp[i].1 = dp[i - 1].1;
        } else {
            dp[i].1 = (dp[i - 1].0 + dp[i - 1].1 * 2) % M;
        }
    }
    println!("{}", dp[n - 1].1);
}
