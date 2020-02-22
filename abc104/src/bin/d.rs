use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
        s: Chars,
    }
    let mut dp = vec![0; 4];
    dp[0] = 1;
    for &c in &s {
        match c {
            'A' => {
                dp[1] = (dp[1] + dp[0]) % M;
            }
            'B' => {
                dp[2] = (dp[2] + dp[1]) % M;
            }
            'C' => {
                dp[3] = (dp[3] + dp[2]) % M;
            }
            _ => {
                dp[3] = (dp[3] * 3 + dp[2]) % M;
                dp[2] = (dp[2] * 3 + dp[1]) % M;
                dp[1] = (dp[1] * 3 + dp[0]) % M;
                dp[0] = (dp[0] * 3) % M;
            }
        }
    }
    println!("{}", dp[3]);
}
